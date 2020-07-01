//!
//! The program resource.
//!

pub mod delete;
pub mod get;
pub mod post;

use std::collections::HashMap;

use zinc_compiler::SourceString;

///
/// The program resource.
///
#[derive(Debug, Clone)]
pub struct Program {
    /// The original program source code, which can be returned back to its authors to be edited.
    pub source: SourceString,
    /// The compiled program entries representation, ready to be run on the Zinc VM.
    pub entries: HashMap<String, zinc_bytecode::Program>,
}

impl Program {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(source: SourceString, entries: HashMap<String, zinc_bytecode::Program>) -> Self {
        Self { source, entries }
    }
}
