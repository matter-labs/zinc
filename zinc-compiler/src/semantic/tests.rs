//!
//! A semantic analyzer tests.
//!

#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error::Error;
use crate::semantic::analyzer::entry::Analyzer as EntryAnalyzer;
use crate::semantic::analyzer::module::Analyzer as ModuleAnalyzer;
use crate::semantic::scope::Scope;
use crate::Parser;

pub static PANIC_TEST_DATA: &str = "Test data is always valid";

static PANIC_SYNTAX_ERROR: &str = "Syntax errors must be eliminated at this point";

pub(crate) fn compile_entry(input: &str) -> Result<(), Error> {
    compile_entry_with_dependencies(input, HashMap::new())
}

pub(crate) fn compile_entry_with_dependencies(
    input: &str,
    dependencies: HashMap<String, Rc<RefCell<Scope>>>,
) -> Result<(), Error> {
    let _intermediate = EntryAnalyzer::default().compile(
        Parser::default()
            .parse(input, None)
            .expect(PANIC_SYNTAX_ERROR),
        dependencies,
    )?;

    Ok(())
}

pub(crate) fn compile_module(input: &str) -> Result<Rc<RefCell<Scope>>, Error> {
    let (scope, _intermediate) = ModuleAnalyzer::new().compile(
        Parser::default()
            .parse(input, None)
            .expect(PANIC_SYNTAX_ERROR),
        HashMap::new(),
    )?;

    Ok(scope)
}
