//!
//! The built-in function type tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::Location;
use crate::semantic::element::r#type::function::builtin::error::Error as BuiltInFunctionError;
use crate::semantic::element::r#type::function::error::Error as FunctionError;
use crate::semantic::Error as SemanticError;

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

    let result = crate::semantic::tests::get_binary_result(input);

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

    let result = crate::semantic::tests::get_binary_result(input);

    assert_eq!(result, expected);
}
