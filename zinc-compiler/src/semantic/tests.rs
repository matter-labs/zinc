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
use crate::Parser;

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
            .expect(crate::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),
        dependencies,
    )?;

    Ok(())
}

pub(crate) fn compile_module(input: &str) -> Result<Rc<RefCell<Scope>>, Error> {
    let (scope, _intermediate) = ModuleAnalyzer::new().compile(
        Parser::default()
            .parse(input, None)
            .expect(crate::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),
    )?;

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

    let expected = Err(Error::Semantic(SemanticError::FunctionMainBeyondEntry {
        location: Location::new(2, 1),
    }));

    let result = crate::semantic::tests::compile_module(input);

    assert_eq!(result, expected);
}

#[test]
fn error_contract_beyond_entry() {
    let input = r#"
contract Uniswap {
    pub fn deposit(amount: u248) -> bool { true }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ContractBeyondEntry {
        location: Location::new(2, 1),
    }));

    let result = crate::semantic::tests::compile_module(input);

    assert_eq!(result, expected);
}
