//!
//! The Zandbox server daemon shared application data.
//!

pub mod contract;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use zksync::web3::types::Address;

use crate::database::client::Client as DatabaseClient;

use self::contract::Contract;

///
/// The Zandbox server daemon shared application data.
///
pub struct SharedData {
    /// The PostgreSQL asynchronous client.
    pub postgresql: DatabaseClient,
    /// The precompiled contracts written at application startup.
    pub contracts: HashMap<Address, Contract>,
}

impl SharedData {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(postgresql: DatabaseClient, contracts: HashMap<Address, Contract>) -> Self {
        Self {
            postgresql,
            contracts,
        }
    }

    ///
    /// Wraps the data into `Arc<Mutex<_>>`.
    ///
    pub fn wrap(self) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(self))
    }
}
