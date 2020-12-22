//!
//! The cached contract data.
//!

use std::collections::HashMap;

use zksync::provider::Provider;

use zinc_vm::Bn256;
use zinc_vm::ContractInput;

use crate::error::Error;
use crate::storage::Storage;

///
/// The cached contract data.
///
#[derive(Debug)]
pub struct LockedContract {
    /// The contract ETH address.
    pub eth_address: zksync_types::Address,
    /// The contract ETH private key.
    pub eth_private_key: zksync_types::H256,

    /// The project name.
    pub name: String,
    /// The project version.
    pub version: semver::Version,
    /// The project instance.
    pub instance: String,

    /// The project JSON representation.
    pub project: zinc_project::Project,
    /// The project bytecode.
    pub bytecode: Vec<u8>,
    /// The project verifying key.
    pub verifying_key: Vec<u8>,

    /// The pre-built contract ready to be called.
    pub build: zinc_types::Contract,
    /// The contract storage.
    pub storage: Storage,
    /// The contract wallet.
    pub wallet: zksync::Wallet<zksync_eth_signer::PrivateKeySigner, zksync::RpcProvider>,

    /// The token used for paying for changing the public key.
    pub change_pubkey_fee_token: zksync_types::Token,
    /// The fee needed for changing the public key.
    pub change_pubkey_fee: num::BigUint,
}

impl LockedContract {
    ///
    /// Initializes a new locked contract.
    ///
    #[allow(clippy::too_many_arguments)]
    pub async fn new(
        network: zksync::Network,

        name: String,
        version: semver::Version,
        instance: String,

        arguments: serde_json::Value,

        project: zinc_project::Project,
        bytecode: Vec<u8>,
        verifying_key: Vec<u8>,

        change_pubkey_fee_token: String,
    ) -> Result<Self, Error> {
        let mut eth_private_key = zksync_types::H256::default();
        eth_private_key.randomize();
        let eth_address: zksync_types::Address =
            zksync_types::tx::PackedEthSignature::address_from_private_key(&eth_private_key)
                .expect(zinc_const::panic::DATA_CONVERSION);

        let application = zinc_types::Application::try_from_slice(bytecode.as_slice())
            .map_err(Error::InvalidBytecode)?;
        let build = match application.clone() {
            zinc_types::Application::Circuit(_circuit) => return Err(Error::NotAContract),
            zinc_types::Application::Contract(contract) => contract,
            zinc_types::Application::Library(_library) => return Err(Error::NotAContract),
        };
        let constructor = build
            .methods
            .get(zinc_const::contract::CONSTRUCTOR_IDENTIFIER)
            .cloned()
            .ok_or(Error::ConstructorNotFound)?;
        let input_value = zinc_types::Value::try_from_typed_json(arguments, constructor.input)
            .map_err(Error::InvalidInput)?;

        let mut storages = HashMap::with_capacity(1);
        storages.insert(
            eth_address,
            Storage::new(build.storage.as_slice()).into_build(),
        );

        let vm_runner = zinc_vm::ContractFacade::new(build.clone());
        let mut output = tokio::task::spawn_blocking(move || {
            vm_runner.run::<Bn256>(ContractInput::new(
                input_value,
                storages,
                zinc_const::contract::CONSTRUCTOR_IDENTIFIER.to_owned(),
                zinc_types::TransactionMsg::default(),
            ))
        })
        .await
        .expect(zinc_const::panic::ASYNC_RUNTIME)
        .map_err(Error::VirtualMachine)?;
        let address = output
            .result
            .into_flat_values()
            .first()
            .cloned()
            .expect(zinc_const::panic::VALIDATED_DURING_RUNTIME_EXECUTION);
        let storage = output
            .storages
            .remove(&address)
            .map(Storage::from_build)
            .expect(zinc_const::panic::VALIDATED_DURING_RUNTIME_EXECUTION);

        let provider = zksync::RpcProvider::new(network);
        let wallet_credentials = zksync::WalletCredentials::from_eth_signer(
            eth_address,
            zksync_eth_signer::PrivateKeySigner::new(eth_private_key),
            network,
        )
        .await?;
        let wallet = zksync::Wallet::new(provider, wallet_credentials).await?;

        let change_pubkey_fee_token = wallet
            .tokens
            .resolve(change_pubkey_fee_token.as_str().into())
            .ok_or(Error::TokenNotFound(change_pubkey_fee_token))?;

        let change_pubkey_fee = zinc_types::num_compat_forward(
            wallet
                .provider
                .get_tx_fee(
                    zksync_types::TxFeeTypes::ChangePubKey {
                        onchain_pubkey_auth: true,
                    },
                    eth_address,
                    change_pubkey_fee_token.id,
                )
                .await?
                .total_fee,
        );

        Ok(Self {
            eth_address,
            eth_private_key,

            name,
            version,
            instance,

            project,
            bytecode,
            verifying_key,

            build,
            storage,
            wallet,

            change_pubkey_fee_token,
            change_pubkey_fee,
        })
    }
}
