//!
//! The Zinc server shared application data.
//!

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use zinc_build::Program as BuildProgram;
use zinc_compiler::State;

use crate::database::client::Client as DatabaseClient;

///
/// The Zinc server shared application data.
///
pub struct SharedData {
    /// The PostgreSQL asynchronous client.
    pub postgresql_client: DatabaseClient,
    /// The precompiled contract templates.
    pub templates: HashMap<i64, BuildProgram>,
}

impl SharedData {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(postgresql_client: DatabaseClient) -> Self {
        Self {
            postgresql_client,
            templates: HashMap::new(),
        }
    }

    ///
    /// Consumes the compiled contract state, inserting the compiled program into the in-memory
    /// contract cache at the specified `key`, which uniquely identifies the contract.
    ///
    pub fn insert_contract(&mut self, key: i64, state: State) {
        self.templates.insert(key, state.into_program(true));
    }

    ///
    /// Wraps the data into `Arc<Mutex<_>>`.
    ///
    pub fn wrap(self) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(self))
    }
}
