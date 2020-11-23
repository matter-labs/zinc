//!
//! The Zandbox server daemon shared application data.
//!

pub mod locked_contract;

use std::collections::HashMap;
use std::sync::RwLock;

use actix_web::web::Data;

use crate::database::client::Client as DatabaseClient;

use self::locked_contract::LockedContract;

///
/// The Zandbox server daemon shared application data.
///
pub struct SharedData {
    /// The PostgreSQL asynchronous client.
    pub postgresql: DatabaseClient,
    /// The zkSync network identifier.
    pub network: zksync::Network,
    /// The contracts waiting to be unlocked by `initialize` endpoint.
    pub locked_contracts: HashMap<zksync_types::Address, LockedContract>,
}

impl SharedData {
    const LOCKED_CONTRACTS_INITIAL_CAPACITY: usize = 64;

    ///
    /// A shortcut constructor.
    ///
    pub fn new(postgresql: DatabaseClient, network: zksync::Network) -> Self {
        Self {
            postgresql,
            network,
            locked_contracts: HashMap::with_capacity(Self::LOCKED_CONTRACTS_INITIAL_CAPACITY),
        }
    }

    ///
    /// Wraps the data into `Arc<Mutex<_>>`.
    ///
    pub fn wrap(self) -> Data<RwLock<Self>> {
        Data::new(RwLock::new(self))
    }
}
