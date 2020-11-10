//!
//! The variable binder tests.
//!

use zinc_lexical::Keyword;
use zinc_lexical::Location;

use crate::error::Error;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_expected_tuple() {
    let input = r#"
fn main() {
    let (a, b, c): (u8, u8) = (1, 2);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::BindingExpectedTuple {
        location: Location::test(3, 9),
        expected: 3,
        found: Type::tuple(
            None,
            vec![Type::integer_unsigned(None, zinc_const::bitlength::BYTE); 2],
        )
        .to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_function_method_self_not_first() {
    let input = r#"
struct Data {
    value: u8,
}

impl Data {
    fn method(value: u8, self) -> u8 {
        value
    }
}

fn main() {
    let data = Data { value: 42 };
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::BindingSelfNotFirstMethodArgument {
            location: Location::test(7, 26),
            name: Keyword::SelfLowercase.to_string(),
            position: 2,
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_function_argument_destructuring_unavailable() {
    let input = r#"
fn main((a, b): (u8, u8)) {}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::BindingFunctionArgumentDestructuringUnavailable {
            location: Location::test(2, 9),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
