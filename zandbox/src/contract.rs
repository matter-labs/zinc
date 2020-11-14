//!
//! The cached contract data.
//!

use zksync::web3::types::Address;
use zksync::web3::types::H256;
use zksync_eth_signer::PrivateKeySigner;
use zksync_types::AccountId;

use zinc_build::Application as BuildApplication;
use zinc_build::Contract as BuildContract;

use crate::database::client::Client as DatabaseClient;
use crate::database::model::contract::select_one::Input as ContractSelectOneInput;
use crate::database::model::field::select::Input as FieldSelectInput;
use crate::database::model::project::select_one::Input as ProjectSelectOneInput;
use crate::error::Error;
use crate::storage::Storage;

///
/// The cached contract data.
///
#[derive(Debug)]
pub struct Contract {
    /// The contract ETH address.
    pub eth_address: Address,
    /// The contract ETH private key.
    pub eth_private_key: H256,
    /// The contract zkSync account ID.
    pub account_id: AccountId,

    /// The contract name.
    pub name: String,
    /// The contract version.
    pub version: semver::Version,
    /// The contract instance.
    pub instance: String,

    /// The contract wallet.
    pub wallet: zksync::Wallet<PrivateKeySigner>,
    /// The pre-built contract ready to be called.
    pub build: BuildContract,
    /// The contract storage.
    pub storage: Storage,
}

impl Contract {
    ///
    /// Loads a contract from the database.
    ///
    pub async fn new(
        network: zksync::Network,
        postgresql: DatabaseClient,
        eth_address: Address,
    ) -> Result<Self, Error> {
        let contract = postgresql
            .select_contract(ContractSelectOneInput::new(eth_address), None)
            .await?;
        let project = postgresql
            .select_project(
                ProjectSelectOneInput::new(
                    contract.name.clone(),
                    semver::Version::parse(contract.version.as_str())
                        .expect(zinc_const::panic::DATA_CONVERSION),
                ),
                None,
            )
            .await?;

        let eth_private_key = zinc_zksync::eth_private_key_from_vec(contract.eth_private_key);

        let provider = zksync::Provider::new(network);
        let wallet_credentials = zksync::WalletCredentials::from_eth_signer(
            eth_address,
            PrivateKeySigner::new(eth_private_key),
            network,
        )
        .await?;
        let wallet = zksync::Wallet::new(provider, wallet_credentials).await?;

        let application = BuildApplication::try_from_slice(project.bytecode.as_slice())
            .expect(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION);
        let build = match application {
            BuildApplication::Circuit(_circuit) => {
                panic!(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION)
            }
            BuildApplication::Contract(contract) => contract,
        };

        let database_fields = postgresql
            .select_fields(
                FieldSelectInput::new(contract.account_id as AccountId),
                None,
            )
            .await?;
        let storage = Storage::new_with_data(
            database_fields,
            build.storage.as_slice(),
            eth_address,
            &wallet,
        )
        .await?;

        Ok(Self {
            eth_address,
            eth_private_key,
            account_id: contract.account_id as AccountId,

            name: contract.name,
            version: semver::Version::parse(contract.version.as_str())
                .expect(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION),
            instance: contract.instance,

            wallet,
            build,
            storage,
        })
    }
}
