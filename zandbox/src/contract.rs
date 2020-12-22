//!
//! The cached contract data.
//!

use std::collections::HashMap;
use std::time::Duration;

use num_old::BigUint;
use num_old::Zero;

use zksync::provider::Provider;

use crate::database::client::Client as DatabaseClient;
use crate::database::model;
use crate::error::Error;
use crate::storage::keeper::Keeper as StorageKeeper;
use crate::storage::Storage;

///
/// The cached contract data.
///
#[derive(Debug)]
pub struct Contract {
    /// The contract ETH address.
    pub eth_address: zksync_types::Address,
    /// The contract ETH private key.
    pub eth_private_key: zksync_types::H256,
    /// The contract zkSync account ID.
    pub account_id: zksync_types::AccountId,

    /// The contract name.
    pub name: String,
    /// The contract version.
    pub version: semver::Version,
    /// The contract instance.
    pub instance: String,

    /// The contract wallet.
    pub wallet: zksync::Wallet<zksync_eth_signer::PrivateKeySigner, zksync::RpcProvider>,
    /// The pre-built contract ready to be called.
    pub build: zinc_types::Contract,
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
        eth_address: zksync_types::Address,
    ) -> Result<Self, Error> {
        let contract = postgresql
            .select_contract(model::contract::select_one::Input::new(eth_address), None)
            .await?;
        let project = postgresql
            .select_project(
                model::project::select_one::Input::new(
                    contract.name.clone(),
                    semver::Version::parse(contract.version.as_str())
                        .expect(zinc_const::panic::DATA_CONVERSION),
                ),
                None,
            )
            .await?;

        let eth_private_key =
            zinc_types::private_key_from_slice(contract.eth_private_key.as_slice());

        let provider = zksync::RpcProvider::new(network);
        let wallet_credentials = zksync::WalletCredentials::from_eth_signer(
            eth_address,
            zksync_eth_signer::PrivateKeySigner::new(eth_private_key),
            network,
        )
        .await?;
        let wallet = zksync::Wallet::new(provider, wallet_credentials).await?;

        let application = zinc_types::Application::try_from_slice(project.bytecode.as_slice())
            .expect(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION);
        let build = match application {
            zinc_types::Application::Circuit(_circuit) => {
                panic!(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION)
            }
            zinc_types::Application::Contract(contract) => contract,
            zinc_types::Application::Library(_library) => {
                panic!(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION)
            }
        };

        let database_fields = postgresql
            .select_fields(
                model::field::select::Input::new(contract.account_id as zksync_types::AccountId),
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
            account_id: contract.account_id as zksync_types::AccountId,

            name: contract.name,
            version: semver::Version::parse(contract.version.as_str())
                .expect(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION),
            instance: contract.instance,

            wallet,
            build,
            storage,
        })
    }

    ///
    /// Runs the contract method on the virtual machine.
    ///
    pub async fn run_method(
        &self,
        method_name: String,
        transaction: zinc_types::TransactionMsg,
        arguments: zinc_types::Value,
        postgresql: DatabaseClient,
    ) -> Result<zinc_vm::ContractOutput, Error> {
        let contract_build = self.build.clone();
        let contract_storage_keeper =
            StorageKeeper::new(postgresql.clone(), self.wallet.provider.network());

        let mut storages = HashMap::with_capacity(1);
        storages.insert(self.eth_address, self.storage.clone().into_build());

        let output = tokio::task::spawn_blocking(move || {
            zinc_vm::ContractFacade::new_with_keeper(
                contract_build,
                Box::new(contract_storage_keeper),
            )
            .run::<zinc_vm::Bn256>(zinc_vm::ContractInput::new(
                arguments,
                storages,
                method_name,
                transaction,
            ))
        })
        .await
        .expect(zinc_const::panic::ASYNC_RUNTIME)
        .map_err(Error::VirtualMachine)?;

        Ok(output)
    }

    ///
    /// Executes the initial deposits batch transaction.
    ///
    /// Initial deposits are needed to unlock accounts created during a contract method execution.
    ///
    pub async fn execute_initial_deposits(
        &self,
        initializers: Vec<zinc_vm::ContractOutputInitializer>,
        nonces: &mut HashMap<zksync_types::Address, u32>,
        transactions: &mut Vec<zinc_types::Transaction>,
    ) -> Result<HashMap<zksync_types::Address, model::contract::insert_one::Input>, Error> {
        if initializers.is_empty() {
            return Ok(HashMap::new());
        }

        let log_id =
            serde_json::to_string(&self.eth_address).expect(zinc_const::panic::DATA_CONVERSION);

        let mut initial_deposit_transactions = Vec::with_capacity(initializers.len());
        for initializer in initializers.iter() {
            let nonce = match nonces.get_mut(&self.eth_address) {
                Some(nonce) => nonce,
                None => {
                    let nonce = self
                        .wallet
                        .provider
                        .account_info(self.eth_address)
                        .await?
                        .committed
                        .nonce;
                    nonces.entry(self.eth_address).or_insert(nonce)
                }
            };
            let token = self
                .wallet
                .tokens
                .resolve(zksync_types::TokenLike::Symbol("ETH".to_owned()))
                .ok_or_else(|| Error::TokenNotFound("ETH".to_owned()))?;
            let amount = BigUint::zero();
            let fee = self
                .wallet
                .provider
                .get_tx_fee(
                    zksync_types::TxFeeTypes::Transfer,
                    initializer.eth_address,
                    token.id,
                )
                .await?
                .total_fee;

            log::info!(
                "[{}] Sending {} {} from {} to {} with fee {} {}",
                log_id,
                zksync_utils::format_units(&amount, token.decimals),
                token.symbol,
                serde_json::to_string(&self.eth_address).expect(zinc_const::panic::DATA_CONVERSION),
                serde_json::to_string(&initializer.eth_address)
                    .expect(zinc_const::panic::DATA_CONVERSION),
                zksync_utils::format_units(&fee, token.decimals),
                token.symbol,
            );

            let (transfer, signature) = self
                .wallet
                .signer
                .sign_transfer(token, amount, fee, initializer.eth_address, *nonce)
                .await?;
            initial_deposit_transactions.push(zinc_types::Transaction::new(
                zksync_types::ZkSyncTx::Transfer(Box::new(transfer)),
                signature,
            ));

            *nonce += 1;
        }

        self.execute_batch(initial_deposit_transactions, 10, 200)
            .await?;

        let mut created_instances = HashMap::with_capacity(initializers.len());
        for initializer in initializers.into_iter() {
            let wallet_credentials = zksync::WalletCredentials::from_eth_signer(
                initializer.eth_address,
                zksync_eth_signer::PrivateKeySigner::new(initializer.eth_private_key),
                self.wallet.provider.network(),
            )
            .await?;
            let mut wallet =
                zksync::Wallet::new(self.wallet.provider.clone(), wallet_credentials).await?;
            let nonce = nonces.entry(initializer.eth_address).or_insert(0);

            let account_id = zksync::utils::wait_for_account_id(&mut wallet, 10_000)
                .await
                .ok_or(Error::AccountIdNotFound)?;

            let token = wallet
                .tokens
                .resolve(zksync_types::TokenLike::Symbol("ETH".to_owned()))
                .ok_or_else(|| Error::TokenNotFound("ETH".to_owned()))?;

            log::info!(
                "[{}] Changing the public key of {}",
                log_id,
                serde_json::to_string(&initializer.eth_address)
                    .expect(zinc_const::panic::DATA_CONVERSION),
            );

            let change_pubkey = wallet
                .signer
                .sign_change_pubkey_tx(*nonce, false, token, BigUint::zero())
                .await?;
            transactions.push(zinc_types::Transaction::new(
                zksync_types::ZkSyncTx::ChangePubKey(Box::new(change_pubkey)),
                None,
            ));

            *nonce += 1;

            created_instances.insert(
                initializer.eth_address,
                model::contract::insert_one::Input::new(
                    account_id,
                    initializer.name,
                    initializer.version,
                    serde_json::to_string(&initializer.eth_address)
                        .expect(zinc_const::panic::DATA_CONVERSION),
                    initializer.eth_address,
                    initializer.eth_private_key,
                ),
            );
        }

        Ok(created_instances)
    }

    ///
    /// Executes the main batch transaction.
    ///
    /// Includes the client transfer and transfers performs during the contract method execution.
    ///
    pub async fn execute_main_batch(
        &self,
        postgresql: DatabaseClient,
        transfers: Vec<zinc_types::TransactionMsg>,
        mut transactions: Vec<zinc_types::Transaction>,
        mut nonces: HashMap<zksync_types::Address, u32>,
        mut eth_private_keys: HashMap<zksync_types::Address, zksync_types::H256>,
    ) -> Result<(), Error> {
        let log_id =
            serde_json::to_string(&self.eth_address).expect(zinc_const::panic::DATA_CONVERSION);

        for transfer in transfers.into_iter() {
            let eth_private_key = match eth_private_keys.get(&transfer.sender).cloned() {
                Some(eth_private_key) => eth_private_key,
                None => {
                    let eth_private_key = postgresql
                        .select_contract(
                            model::contract::select_one::Input::new(transfer.sender),
                            None,
                        )
                        .await
                        .map(|contract| {
                            zinc_types::private_key_from_slice(contract.eth_private_key.as_slice())
                        })?;
                    eth_private_keys.insert(transfer.sender, eth_private_key);
                    eth_private_key
                }
            };

            let wallet_credentials = zksync::WalletCredentials::from_eth_signer(
                transfer.sender,
                zksync_eth_signer::PrivateKeySigner::new(eth_private_key),
                self.wallet.provider.network(),
            )
            .await?;
            let wallet =
                zksync::Wallet::new(self.wallet.provider.clone(), wallet_credentials).await?;

            let nonce = match nonces.get_mut(&transfer.sender) {
                Some(nonce) => nonce,
                None => {
                    let nonce = wallet
                        .provider
                        .account_info(transfer.sender)
                        .await?
                        .committed
                        .nonce;
                    nonces.entry(transfer.sender).or_insert(nonce)
                }
            };
            let token = wallet
                .tokens
                .resolve(transfer.token_address.into())
                .ok_or_else(|| {
                    Error::TokenNotFound(
                        serde_json::to_string(&transfer.token_address)
                            .expect(zinc_const::panic::DATA_CONVERSION),
                    )
                })?;
            let amount = zksync::utils::closest_packable_token_amount(&transfer.amount);
            let fee = BigUint::zero();

            log::info!(
                "[{}] Sending {} {} from {} to {}",
                log_id,
                zksync_utils::format_units(&amount, token.decimals),
                token.symbol,
                serde_json::to_string(&transfer.sender).expect(zinc_const::panic::DATA_CONVERSION),
                serde_json::to_string(&transfer.recipient)
                    .expect(zinc_const::panic::DATA_CONVERSION),
            );

            let (transfer, signature) = wallet
                .signer
                .sign_transfer(token, amount, fee, transfer.recipient, *nonce)
                .await?;
            transactions.push(zinc_types::Transaction::new(
                zksync_types::ZkSyncTx::Transfer(Box::new(transfer)),
                signature,
            ));

            *nonce += 1;
        }

        self.execute_batch(transactions, 10, 200).await?;

        Ok(())
    }

    ///
    /// Executes a batch transaction.
    ///
    async fn execute_batch(
        &self,
        transactions: Vec<zinc_types::Transaction>,
        commit_timeout_secs: u64,
        polling_interval_millis: u64,
    ) -> Result<(), Error> {
        let handles: Vec<zksync::operations::SyncTransactionHandle<zksync::RpcProvider>> = self
            .wallet
            .provider
            .send_txs_batch(
                transactions
                    .into_iter()
                    .map(|transaction| {
                        (
                            transaction.tx,
                            transaction
                                .ethereum_signature
                                .map(|signature| signature.signature),
                        )
                    })
                    .collect(),
                None,
            )
            .await?
            .into_iter()
            .map(|tx_hash| {
                let mut handle = zksync::operations::SyncTransactionHandle::new(
                    tx_hash,
                    self.wallet.provider.clone(),
                )
                .commit_timeout(Duration::from_secs(commit_timeout_secs));
                handle
                    .polling_interval(Duration::from_millis(polling_interval_millis))
                    .expect(zinc_const::panic::DATA_CONVERSION);
                handle
            })
            .collect();

        if let Some(handle) = handles.last() {
            let tx_info = handle.wait_for_commit().await?;
            if !tx_info.success.unwrap_or_default() {
                return Err(Error::TransferFailure(
                    tx_info
                        .fail_reason
                        .unwrap_or_else(|| "Unknown error".to_owned()),
                ));
            }
        }

        Ok(())
    }
}
