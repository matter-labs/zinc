//!
//! The module semantic analyzer.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::semantic::analyzer::statement::Analyzer as StatementAnalyzer;
use crate::semantic::analyzer::statement::Context as StatementAnalyzerContext;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;
use crate::source::module::Module as SourceModule;

///
/// Analyzes a module, which are located in non-`main.zn` files.
///
/// To analyze the project entry, use the entry analyzer.
///
pub struct Analyzer {}

impl Analyzer {
    pub fn analyze(module: SourceModule) -> Result<Rc<RefCell<Scope>>, Error> {
        let (module, dependencies) = match module {
            SourceModule::File(file) => (file.tree, HashMap::new()),
            SourceModule::Directory(directory) => (directory.entry.tree, directory.modules),
        };

        StatementAnalyzer::module(module, dependencies, StatementAnalyzerContext::Module)
    }
}
