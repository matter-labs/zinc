//!
//! A semantic analyzer tests.
//!

#![allow(dead_code)]
#![allow(unused_imports)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::analyzer::entry::Analyzer as EntryAnalyzer;
use crate::semantic::analyzer::module::Analyzer as ModuleAnalyzer;
use crate::semantic::error::Error as SemanticError;
use crate::semantic::scope::Scope;
use crate::source::module::Module as SourceModule;
use crate::source::Source;
use crate::syntax::tree::module::Module as SyntaxModule;
use crate::Parser;

pub(crate) fn compile_entry(input: &str) -> Result<(), Error> {
    compile_entry_with_dependencies(input, HashMap::new())
}

pub(crate) fn compile_entry_with_dependencies(
    input: &str,
    dependencies: HashMap<String, SourceModule>,
) -> Result<(), Error> {
    let source = Source::test(input, dependencies)?;
    EntryAnalyzer::analyze(source.entry.tree, source.modules)?;

    Ok(())
}

pub(crate) fn compile_module(input: &str) -> Result<Rc<RefCell<Scope>>, Error> {
    compile_module_with_dependencies(input, HashMap::new())
}

pub(crate) fn compile_module_with_dependencies(
    input: &str,
    dependencies: HashMap<String, SourceModule>,
) -> Result<Rc<RefCell<Scope>>, Error> {
    let scope = ModuleAnalyzer::analyze(SourceModule::test(input, dependencies)?)?;

    Ok(scope)
}

#[test]
fn error_entry_point_missing() {
    let input = r#"
fn another() -> u8 {
    42
}
"#;

    let expected = Err(Error::Semantic(SemanticError::EntryPointMissing));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_entry_point_ambiguous() {
    let input = r#"
fn main() -> u8 {
    42
}

contract Uniswap {
    pub fn deposit(amount: u248) -> bool { true }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::EntryPointAmbiguous {
        main: Location::new(2, 1),
        contract: Location::new(6, 1),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_entry_point_constant_function_main() {
    let input = r#"
const fn main() -> u8 {
    42
}
"#;

    let expected = Err(Error::Semantic(SemanticError::EntryPointConstant {
        location: Location::new(2, 1),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_entry_point_constant_contract_function() {
    let input = r#"
contract Uniswap {
    pub const fn deposit(amount: u248) -> bool { true }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::EntryPointConstant {
        location: Location::new(3, 5),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_function_main_beyond_entry() {
    let input = r#"
fn main() -> u8 {
    42
}
"#;

    let expected = Error::Semantic(SemanticError::FunctionMainBeyondEntry {
        location: Location::new(2, 1),
    });

    let result = crate::semantic::tests::compile_module(input).unwrap_err();

    assert_eq!(result, expected);
}

#[test]
fn error_contract_beyond_entry() {
    let input = r#"
contract Uniswap {
    pub fn deposit(amount: u248) -> bool { true }
}
"#;

    let expected = Error::Semantic(SemanticError::ContractBeyondEntry {
        location: Location::new(2, 1),
    });

    let result = crate::semantic::tests::compile_module(input).unwrap_err();

    assert_eq!(result, expected);
}

#[test]
fn error_module_file_not_found() {
    let input = r#"
mod unknown;

fn main() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::ModuleFileNotFound {
        location: Location::new(2, 5),
        name: "unknown".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
