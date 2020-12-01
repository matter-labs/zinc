//!
//! The Zandbox server daemon contract storage keeper.
//!

use num::BigInt;

use crate::database::client::Client as DatabaseClient;
use crate::database::model;
use crate::storage::Storage;

pub struct Keeper {
    /// The PostgreSQL asynchronous client.
    pub postgresql: DatabaseClient,
    /// The zkSync network identifier.
    pub network: zksync::Network,
}

impl Keeper {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(postgresql: DatabaseClient, network: zksync::Network) -> Self {
        Self {
            postgresql,
            network,
        }
    }
}

impl zinc_vm::IContractStorageKeeper for Keeper {
    fn fetch(
        &self,
        eth_address: BigInt,
        field_types: Vec<zinc_types::ContractFieldType>,
    ) -> Result<zinc_types::Value, zinc_vm::Error> {
        let mut runtime = tokio::runtime::Builder::new()
            .threaded_scheduler()
            .core_threads(1)
            .enable_io()
            .build()
            .expect(zinc_const::panic::ASYNC_RUNTIME);

        let eth_address = zinc_types::address_from_slice(eth_address.to_bytes_be().1.as_slice());
        let contract = runtime.block_on(
            self.postgresql
                .select_contract(model::contract::select_one::Input::new(eth_address), None),
        )?;

        let fields = runtime.block_on(self.postgresql.select_fields(
            model::field::select::Input::new(contract.account_id as zksync_types::AccountId),
            None,
        ))?;
        let eth_private_key =
            zinc_types::private_key_from_slice(contract.eth_private_key.as_slice());

        let provider = zksync::Provider::new(self.network);
        let wallet_credentials = runtime.block_on(zksync::WalletCredentials::from_eth_signer(
            eth_address,
            zksync_eth_signer::PrivateKeySigner::new(eth_private_key),
            self.network,
        ))?;
        let wallet = runtime.block_on(zksync::Wallet::new(provider, wallet_credentials))?;

        let storage = runtime.block_on(Storage::new_with_data(
            fields,
            field_types.as_slice(),
            eth_address,
            &wallet,
        ))?;

        Ok(storage.into_build())
    }
}
