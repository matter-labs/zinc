//!
//! The intrinsic function tests.
//!

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::error::Error as TypeError;
use crate::semantic::element::r#type::function::error::Error as FunctionError;
use crate::semantic::element::r#type::function::intrinsic::assert::Function as IntrinsicAssertFunction;
use crate::semantic::element::r#type::function::intrinsic::debug::Function as IntrinsicDebugFunction;
use crate::semantic::element::r#type::function::intrinsic::error::Error as IntrinsicFunctionError;
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

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::Intrinsic(
            IntrinsicFunctionError::SpecifierMissing {
                location: Location::test(3, 5),
                function: "assert",
            },
        )),
    ))));

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

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::Intrinsic(IntrinsicFunctionError::Unknown {
            location: Location::test(5, 13),
            function: "unknown".to_owned(),
        })),
    ))));

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

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::Intrinsic(
            IntrinsicFunctionError::DebugArgumentCount {
                location: Location::test(3, 5),
                expected: 3,
                found: 2,
            },
        )),
    ))));

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

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::Intrinsic(
            IntrinsicFunctionError::DebugArgumentCount {
                location: Location::test(3, 5),
                expected: 2,
                found: 3,
            },
        )),
    ))));

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

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::ArgumentType {
            location: Location::test(3, 10),
            function: "dbg".to_owned(),
            name: "format".to_owned(),
            position: IntrinsicDebugFunction::ARGUMENT_INDEX_FORMAT + 1,
            expected: Type::string(None).to_string(),
            found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
        }),
    ))));

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

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::ArgumentCount {
            location: Location::test(3, 5),
            function: "assert".to_owned(),
            expected: IntrinsicAssertFunction::ARGUMENT_COUNT_MANDATORY,
            found: IntrinsicAssertFunction::ARGUMENT_COUNT_MANDATORY - 1,
            reference: None,
        }),
    ))));

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

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::ArgumentCount {
            location: Location::test(3, 5),
            function: "assert".to_owned(),
            expected: IntrinsicAssertFunction::ARGUMENT_COUNT_OPTIONAL,
            found: IntrinsicAssertFunction::ARGUMENT_COUNT_OPTIONAL + 1,
            reference: None,
        }),
    ))));

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

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::ArgumentType {
            location: Location::test(3, 13),
            function: "assert".to_owned(),
            name: "condition".to_owned(),
            position: IntrinsicAssertFunction::ARGUMENT_INDEX_CONDITION + 1,
            expected: Type::boolean(None).to_string(),
            found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
        }),
    ))));

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

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::ArgumentType {
            location: Location::test(3, 19),
            function: "assert".to_owned(),
            name: "message".to_owned(),
            position: IntrinsicAssertFunction::ARGUMENT_INDEX_MESSAGE + 1,
            expected: Type::string(None).to_string(),
            found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
