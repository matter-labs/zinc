//!
//! The `dbg!` intrinsic function tests.
//!

use zinc_lexical::Location;

use crate::error::Error;
use crate::semantic::element::r#type::function::intrinsic::debug::Function as DebugFunction;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_argument_count_lesser() {
    let input = r#"
fn main() {
    dbg!("{} {}", 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionDebugArgumentCount {
        location: Location::test(3, 5),
        expected: 3,
        found: 2,
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_argument_count_greater() {
    let input = r#"
fn main() {
    dbg!("{}", 42, 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionDebugArgumentCount {
        location: Location::test(3, 5),
        expected: 2,
        found: 3,
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_argument_1_format_expected_string() {
    let input = r#"
fn main() {
    dbg!(42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(3, 10),
        function: DebugFunction::IDENTIFIER.to_owned(),
        name: "format".to_owned(),
        position: DebugFunction::ARGUMENT_INDEX_FORMAT + 1,
        expected: Type::string(None).to_string(),
        found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
