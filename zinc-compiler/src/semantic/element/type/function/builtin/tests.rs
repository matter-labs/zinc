//!
//! The built-in function tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::error::Error as TypeError;
use crate::semantic::element::r#type::function::builtin::assert::Function as BuiltInAssertFunction;
use crate::semantic::element::r#type::function::builtin::debug::Function as BuiltInDebugFunction;
use crate::semantic::element::r#type::function::builtin::error::Error as BuiltInFunctionTypeError;
use crate::semantic::element::r#type::function::error::Error as FunctionTypeError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Error as ElementError;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_specifier_missing() {
    let input = r#"
fn main() {
    assert();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 11),
        ElementError::Type(TypeError::Function(FunctionTypeError::BuiltIn(
            BuiltInFunctionTypeError::specifier_missing("assert"),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 13),
        ElementError::Type(TypeError::Function(FunctionTypeError::BuiltIn(
            BuiltInFunctionTypeError::unknown("unknown".to_owned()),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_debug_argument_count_lesser() {
    let input = r#"
fn main() {
    dbg!("{} {}", 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 9),
        ElementError::Type(TypeError::Function(FunctionTypeError::BuiltIn(
            BuiltInFunctionTypeError::debug_argument_count(3, 2),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_debug_argument_count_greater() {
    let input = r#"
fn main() {
    dbg!("{}", 42, 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 9),
        ElementError::Type(TypeError::Function(FunctionTypeError::BuiltIn(
            BuiltInFunctionTypeError::debug_argument_count(2, 3),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_debug_argument_1_format_expected_string() {
    let input = r#"
fn main() {
    dbg!(42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 9),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "dbg".to_owned(),
            "format".to_owned(),
            BuiltInDebugFunction::ARGUMENT_INDEX_FORMAT_STRING + 1,
            Type::string().to_string(),
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_assert_argument_count_lesser() {
    let input = r#"
fn main() {
    assert!();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 12),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "assert".to_owned(),
            BuiltInAssertFunction::ARGUMENT_COUNT_MANDATORY,
            BuiltInAssertFunction::ARGUMENT_COUNT_MANDATORY - 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_assert_argument_count_greater() {
    let input = r#"
fn main() {
    assert!(true, "default", 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 12),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "assert".to_owned(),
            BuiltInAssertFunction::ARGUMENT_COUNT_OPTIONAL,
            BuiltInAssertFunction::ARGUMENT_COUNT_OPTIONAL + 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_assert_argument_1_condition_expected_boolean() {
    let input = r#"
fn main() {
    assert!(42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 12),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "assert".to_owned(),
            "condition".to_owned(),
            BuiltInAssertFunction::ARGUMENT_INDEX_CONDITION + 1,
            Type::boolean().to_string(),
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_assert_argument_2_message_expected_string() {
    let input = r#"
fn main() {
    assert!(true, 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 12),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "assert".to_owned(),
            "message".to_owned(),
            BuiltInAssertFunction::ARGUMENT_INDEX_MESSAGE + 1,
            Type::string().to_string(),
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
