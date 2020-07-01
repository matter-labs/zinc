//!
//! The Zinc server shared application data.
//!

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use zinc_compiler::SourceString;

use crate::program::Program;

///
/// The Zinc server shared application data.
///
#[derive(Debug, Default, Clone)]
pub struct AppData {
    /// The published programs storage.
    pub programs: HashMap<String, Program>,
}

impl AppData {
    /// The program storage initial capacity.
    const PROGRAMS_INITIAL_CAPACITY: usize = 256;

    ///
    /// A shortcut constructor.
    ///
    pub fn new() -> Self {
        Self {
            programs: HashMap::with_capacity(Self::PROGRAMS_INITIAL_CAPACITY),
        }
    }

    ///
    /// Gets the program source code from the storage.
    ///
    pub fn get_program_source(&self, name: &str) -> Option<SourceString> {
        self.programs
            .get(name)
            .map(|program| program.source.to_owned())
    }

    ///
    /// Insert a program into the storage.
    ///
    pub fn insert_program(&mut self, name: String, program: Program) {
        self.programs.insert(name, program);
    }

    ///
    /// Removes a program from the storage.
    ///
    pub fn remove_program(&mut self, name: &str) -> Option<SourceString> {
        self.programs.remove(name).map(|program| program.source)
    }

    ///
    /// Checks if the program exists in the storage.
    ///
    pub fn contains(&self, name: &str) -> bool {
        self.programs.contains_key(name)
    }

    ///
    /// Wraps the data into `Arc<Mutex<_>>`.
    ///
    pub fn wrap(self) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(self))
    }
}
