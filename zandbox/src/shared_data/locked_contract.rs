//!
//! The cached contract data.
//!

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
    pub project: zinc_source::Project,
    /// The project bytecode.
    pub bytecode: Vec<u8>,
    /// The project verifying key.
    pub verifying_key: Vec<u8>,

    /// The pre-built contract ready to be called.
    pub build: zinc_build::Contract,
    /// The contract storage.
    pub storage: Storage,
    /// The contract wallet.
    pub wallet: zksync::Wallet<zksync_eth_signer::PrivateKeySigner>,
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

        project: zinc_source::Project,
        bytecode: Vec<u8>,
        verifying_key: Vec<u8>,
    ) -> Result<Self, Error> {
        let mut eth_private_key = zksync_types::H256::default();
        eth_private_key.randomize();
        let eth_address: zksync_types::Address =
            zksync_types::tx::PackedEthSignature::address_from_private_key(&eth_private_key)
                .expect(zinc_const::panic::DATA_CONVERSION);

        let application = zinc_build::Application::try_from_slice(bytecode.as_slice())
            .map_err(Error::InvalidBytecode)?;
        let build = match application.clone() {
            zinc_build::Application::Circuit(_circuit) => return Err(Error::NotAContract),
            zinc_build::Application::Contract(contract) => contract,
        };
        let constructor = build
            .methods
            .get(zinc_const::contract::CONSTRUCTOR_NAME)
            .cloned()
            .ok_or(Error::ConstructorNotFound)?;
        let input_value = zinc_build::Value::try_from_typed_json(arguments, constructor.input)
            .map_err(Error::InvalidInput)?;
        let storage = Storage::new(build.storage.as_slice()).into_build();

        let vm_runner = zinc_vm::ContractFacade::new(build.clone());
        let output = async_std::task::spawn_blocking(move || {
            vm_runner.run::<Bn256>(ContractInput::new(
                input_value,
                storage,
                zinc_const::contract::CONSTRUCTOR_NAME.to_owned(),
                zinc_zksync::TransactionMsg::default(),
            ))
        })
        .await
        .map_err(Error::VirtualMachine)?;
        let storage = Storage::from_build(output.result);

        let provider = zksync::Provider::new(network);
        let wallet_credentials = zksync::WalletCredentials::from_eth_signer(
            eth_address,
            zksync_eth_signer::PrivateKeySigner::new(eth_private_key),
            network,
        )
        .await?;
        let wallet = zksync::Wallet::new(provider, wallet_credentials).await?;

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
        })
    }
}
