//!
//! The test function tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::error::Error as TypeError;
use crate::semantic::element::r#type::function::error::Error as FunctionError;
use crate::semantic::element::r#type::function::test::error::Error as TestFunctionError;
use crate::semantic::element::Error as ElementError;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_call_forbidden() {
    let input = r#"
#[test]
fn test() {
    assert!(true);
}

fn main() {
    let value = test();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::Test(TestFunctionError::CallForbidden {
            location: Location::test(8, 17),
            function: "test".to_owned(),
        })),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_beyond_module_scope() {
    let input = r#"
struct Data {
    value: u8,
}

impl Data {
    #[test]
    fn test() {
        assert!(true);
    }
}

fn main() -> u8 {
    42
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::Test(TestFunctionError::BeyondModuleScope {
            location: Location::test(8, 5),
            function: "test".to_owned(),
        })),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_public_forbidden() {
    let input = r#"
#[test]
pub fn test() {
    assert!(true);
}

fn main() {
    let value = test();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::Test(TestFunctionError::PublicForbidden {
            location: Location::test(3, 1),
            function: "test".to_owned(),
        })),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_constant_forbidden() {
    let input = r#"
#[test]
const fn test() {
    assert!(true);
}

fn main() {
    let value = test();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::Test(TestFunctionError::ConstantForbidden {
            location: Location::test(3, 1),
            function: "test".to_owned(),
        })),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_cannot_have_arguments() {
    let input = r#"
#[test]
fn test(value: u8) {
    assert!(true);
}

fn main() {
    let value = test();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::Test(
            TestFunctionError::CannotHaveArguments {
                location: Location::test(3, 1),
                function: "test".to_owned(),
            },
        )),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_cannot_return_value() {
    let input = r#"
#[test]
fn test() -> u8 {
    assert!(true);
    42
}

fn main() {
    let value = test();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::Test(TestFunctionError::CannotReturnValue {
            location: Location::test(3, 1),
            function: "test".to_owned(),
        })),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
