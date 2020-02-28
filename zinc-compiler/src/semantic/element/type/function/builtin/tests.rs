//!
//! The built-in function tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::Location;
use crate::semantic::element::r#type::function::builtin::error::Error as BuiltInFunctionError;
use crate::semantic::element::r#type::function::error::Error as FunctionError;
use crate::semantic::Error as SemanticError;

#[test]
fn error_specifier_missing() {
    let input = r#"
fn main() {
    assert();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Function(
        Location::new(3, 11),
        FunctionError::BuiltIn(BuiltInFunctionError::SpecifierMissing("assert")),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_unknown() {
    let input = r#"
fn unknown() {}

fn main() {
    unknown!();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Function(
        Location::new(5, 13),
        FunctionError::BuiltIn(BuiltInFunctionError::Unknown("unknown".to_owned())),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_debug_argument_count_lesser() {
    let input = r#"
fn main() {
    dbg!("{} {}", 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Function(
        Location::new(3, 9),
        FunctionError::BuiltIn(BuiltInFunctionError::DebugArgumentCount(3, 2)),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_debug_argument_count_greater() {
    let input = r#"
fn main() {
    dbg!("{}", 42, 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Function(
        Location::new(3, 9),
        FunctionError::BuiltIn(BuiltInFunctionError::DebugArgumentCount(2, 3)),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}
