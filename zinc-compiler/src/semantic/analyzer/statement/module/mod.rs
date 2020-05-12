//!
//! The `mod` statement semantic analyzer.
//!

mod tests;

use std::cell::RefCell;
use std::rc::Rc;

use crate::semantic::analyzer::module::Analyzer as ModuleAnalyzer;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;
use crate::source::module::Module as SourceModule;

pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes a compile-time only module declaration statement.
    ///
    pub fn analyze(module: SourceModule) -> Result<Rc<RefCell<Scope>>, Error> {
        ModuleAnalyzer::analyze(module)
    }
}
