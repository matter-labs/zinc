//!
//! The function type tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::Location;
use crate::semantic::element::r#type::function::error::Error as FunctionError;
use crate::semantic::element::r#type::Type;
use crate::semantic::Error as SemanticError;

#[test]
fn error_argument_count() {
    let input = r#"
fn another(x: u8) -> u8 {
    42
}

fn main() {
    let value = another();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Function(
        Location::new(7, 24),
        FunctionError::ArgumentCount("another".to_owned(), 1, 0),
    )));

    let result = crate::semantic::tests::get_binary_result(input);

    assert_eq!(result, expected);
}

#[test]
fn error_argument_type() {
    let input = r#"
fn another(x: u8) -> u8 {
    42
}

fn main() {
    let value = another(false);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Function(
        Location::new(7, 24),
        FunctionError::ArgumentType(
            "another".to_owned(),
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            1,
            "x".to_owned(),
            Type::boolean().to_string(),
        ),
    )));

    let result = crate::semantic::tests::get_binary_result(input);

    assert_eq!(result, expected);
}

#[test]
fn error_argument_constantness() {
    let input = r#"
fn main() -> [u8; 2] {
    let array = [1, 2, 3, 4];
    let new_length = 2;
    std::array::truncate(array, new_length)
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Function(
        Location::new(5, 25),
        FunctionError::ArgumentConstantness(
            "truncate".to_owned(),
            2,
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        ),
    )));

    let result = crate::semantic::tests::get_binary_result(input);

    assert_eq!(result, expected);
}

#[test]
fn error_argument_not_evaluable() {
    let input = r#"
fn another(x: u8) -> u8 {
    42
}

type X = u8;

fn main() {
    let value = another(X);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Function(
        Location::new(9, 24),
        FunctionError::ArgumentNotEvaluable(
            "another".to_owned(),
            1,
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        ),
    )));

    let result = crate::semantic::tests::get_binary_result(input);

    assert_eq!(result, expected);
}

#[test]
fn error_return_type() {
    let input = r#"
fn another() -> bool {
    42
}

fn main() {
    let value = another();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Function(
        Location::new(2, 17),
        FunctionError::ReturnType(
            "another".to_owned(),
            Type::boolean().to_string(),
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        ),
    )));

    let result = crate::semantic::tests::get_binary_result(input);

    assert_eq!(result, expected);
}

#[test]
fn error_non_callable_object() {
    let input = r#"
type another = (u8, u8);

fn main() {
    let value = another();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Function(
        Location::new(5, 24),
        FunctionError::NonCallableObject(
            Type::tuple(vec![Type::integer_unsigned(crate::BITLENGTH_BYTE); 2]).to_string(),
        ),
    )));

    let result = crate::semantic::tests::get_binary_result(input);

    assert_eq!(result, expected);
}

#[test]
fn error_builtin_specifier_missing() {
    let input = r#"
fn main() {
    assert();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Function(
        Location::new(3, 11),
        FunctionError::BuiltInSpecifierMissing("assert"),
    )));

    let result = crate::semantic::tests::get_binary_result(input);

    assert_eq!(result, expected);
}

#[test]
fn error_builtin_unknown() {
    let input = r#"
fn unknown() {}

fn main() {
    unknown!();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Function(
        Location::new(5, 13),
        FunctionError::BuiltInUnknown("unknown".to_owned()),
    )));

    let result = crate::semantic::tests::get_binary_result(input);

    assert_eq!(result, expected);
}
