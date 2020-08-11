//!
//! The Zinc server shared application data.
//!

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use zinc_bytecode::Program as BytecodeProgram;
use zinc_compiler::State;
use zinc_postgres::Client as PostgresqlClient;

///
/// The Zinc server shared application data.
///
pub struct SharedData {
    /// The PostgreSQL async client.
    pub postgresql_client: PostgresqlClient,
    /// The precompiled contract methods.
    pub contracts: HashMap<i64, BytecodeProgram>,
}

impl SharedData {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(postgresql_client: PostgresqlClient) -> Self {
        Self {
            postgresql_client,
            contracts: HashMap::new(),
        }
    }

    ///
    /// Consumes the compiled contract state, inserting the compiled program into the in-memory
    /// contract cache at the specified `key`, which uniquely identifies the contract.
    ///
    pub fn insert_contract(&mut self, key: i64, state: State) {
        self.contracts.insert(key, state.into_program(true));
    }

    ///
    /// Wraps the data into `Arc<Mutex<_>>`.
    ///
    pub fn wrap(self) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(self))
    }
}
