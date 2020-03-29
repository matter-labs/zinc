//!
//! A semantic analyzer test.
//!

#![allow(dead_code)]

pub static PANIC_TEST_DATA: &str = "Test data is always valid";

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error::Error;
use crate::EntryPointAnalyzer;
use crate::ModuleAnalyzer;
use crate::Parser;
use crate::Scope;

static PANIC_SYNTAX_ERROR: &str = "Syntax errors must be eliminated at this point";

pub(crate) fn compile_entry(input: &str) -> Result<(), Error> {
    compile_entry_with_dependencies(input, HashMap::new())
}

pub(crate) fn compile_entry_with_dependencies(
    input: &str,
    dependencies: HashMap<String, Rc<RefCell<Scope>>>,
) -> Result<(), Error> {
    let _representation = EntryPointAnalyzer::default().compile(
        Parser::default()
            .parse(input, None)
            .expect(PANIC_SYNTAX_ERROR),
        dependencies,
    )?;
    Ok(())
}

pub(crate) fn compile_module(input: &str) -> Result<Rc<RefCell<Scope>>, Error> {
    let (scope, _representation) = ModuleAnalyzer::new().compile(
        Parser::default()
            .parse(input, None)
            .expect(PANIC_SYNTAX_ERROR),
    )?;
    Ok(scope)
}
