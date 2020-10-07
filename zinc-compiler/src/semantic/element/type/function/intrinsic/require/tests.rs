//!
//! The intrinsic function tests.
//!

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::error::Error as TypeError;
use crate::semantic::element::r#type::function::error::Error as FunctionError;
use crate::semantic::element::r#type::function::intrinsic::require::Function as RequireFunction;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Error as ElementError;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_argument_count_lesser() {
    let input = r#"
fn main() {
    require();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::ArgumentCount {
            location: Location::test(3, 5),
            function: RequireFunction::IDENTIFIER.to_owned(),
            expected: RequireFunction::ARGUMENT_COUNT_MANDATORY,
            found: RequireFunction::ARGUMENT_COUNT_MANDATORY - 1,
            reference: None,
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_argument_count_greater() {
    let input = r#"
fn main() {
    require(true, "default", 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::ArgumentCount {
            location: Location::test(3, 5),
            function: RequireFunction::IDENTIFIER.to_owned(),
            expected: RequireFunction::ARGUMENT_COUNT_OPTIONAL,
            found: RequireFunction::ARGUMENT_COUNT_OPTIONAL + 1,
            reference: None,
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_argument_1_condition_expected_boolean() {
    let input = r#"
fn main() {
    require(42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::ArgumentType {
            location: Location::test(3, 13),
            function: RequireFunction::IDENTIFIER.to_owned(),
            name: "condition".to_owned(),
            position: RequireFunction::ARGUMENT_INDEX_CONDITION + 1,
            expected: Type::boolean(None).to_string(),
            found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_argument_2_message_expected_string() {
    let input = r#"
fn main() {
    require(true, 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::ArgumentType {
            location: Location::test(3, 19),
            function: RequireFunction::IDENTIFIER.to_owned(),
            name: "message".to_owned(),
            position: RequireFunction::ARGUMENT_INDEX_MESSAGE + 1,
            expected: Type::string(None).to_string(),
            found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
