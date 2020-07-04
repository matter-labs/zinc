//!
//! The program model.
//!

pub mod entry;

use std::collections::HashMap;

use zinc_compiler::SourceString;

use self::entry::Entry;

///
/// The program resource.
///
#[derive(Debug, Clone)]
pub struct Program {
    /// The original program source code, which can be returned back to its authors to be edited.
    pub source: SourceString,
    /// The compiled program entries representation, ready to be run on the Zinc VM.
    pub entries: HashMap<String, Entry>,
}

impl Program {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(source: SourceString, entries: HashMap<String, Entry>) -> Self {
        Self { source, entries }
    }

    ///
    /// Returns a program entry.
    ///
    pub fn get_entry(&self, name: &str) -> Option<&Entry> {
        self.entries.get(name)
    }
}
