//!
//! The Zandbox server daemon shared application data.
//!

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use zinc_build::Contract as BuildContract;

use crate::database::client::Client as DatabaseClient;

///
/// The Zandbox server daemon shared application data.
///
pub struct SharedData {
    /// The PostgreSQL asynchronous client.
    pub postgresql_client: DatabaseClient,
    /// The precompiled contracts written at application startup.
    pub contracts: HashMap<i64, BuildContract>,
}

impl SharedData {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(postgresql_client: DatabaseClient, contracts: HashMap<i64, BuildContract>) -> Self {
        Self {
            postgresql_client,
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
