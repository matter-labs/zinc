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
    /// The precompiled program entries.
    pub programs: HashMap<i32, HashMap<String, BytecodeProgram>>,
}

impl SharedData {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(postgresql_client: PostgresqlClient) -> Self {
        Self {
            postgresql_client,
            programs: HashMap::new(),
        }
    }

    ///
    /// Consumes the compiled program state, appending the entry points to the in-memory
    /// program cache at the specified `key`, which uniquely identifies the program.
    ///
    pub fn append_programs(
        &mut self,
        key: i32,
        compiled: State,
    ) -> HashMap<String, BytecodeProgram> {
        for (name, entry) in compiled.into_entries(true).into_iter() {
            let program = BytecodeProgram::from_bytes(entry.into_bytecode().as_slice())
                .expect(zinc_const::panic::DATA_SERIALIZATION);

            self.programs
                .entry(key)
                .or_insert_with(HashMap::new)
                .insert(name, program);
        }

        self.programs
            .get(&key)
            .cloned()
            .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
    }

    ///
    /// Gets a compiled program entry from the in-memory cache by `program_key` and `entry_name`.
    ///
    pub fn get_entry(&self, key: i32, entry_name: &str) -> Option<BytecodeProgram> {
        self.programs
            .get(&key)
            .and_then(|entries| entries.get(entry_name).cloned())
    }

    ///
    /// Wraps the data into `Arc<Mutex<_>>`.
    ///
    pub fn wrap(self) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(self))
    }
}
