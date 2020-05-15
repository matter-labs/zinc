//!
//! The function tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::error::Error as TypeError;
use crate::semantic::element::r#type::function::error::Error as FunctionTypeError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::element::Error as ElementError;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_argument_count_lesser() {
    let input = r#"
fn another(x: u8) -> u8 {
    42
}

fn main() {
    let value = another();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionTypeError::ArgumentCount {
            location: Location::new(7, 17),
            function: "another".to_owned(),
            expected: 1,
            found: 0,
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_argument_count_greater() {
    let input = r#"
fn another(x: u8) -> u8 {
    42
}

fn main() {
    let value = another(1, 2);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionTypeError::ArgumentCount {
            location: Location::new(7, 17),
            function: "another".to_owned(),
            expected: 1,
            found: 2,
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

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

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionTypeError::ArgumentType {
            location: Location::new(7, 25),
            function: "another".to_owned(),
            name: "x".to_owned(),
            position: 1,
            expected: Type::integer_unsigned(None, crate::BITLENGTH_BYTE).to_string(),
            found: Type::boolean(None).to_string(),
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

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

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionTypeError::ArgumentConstantness {
            location: Location::new(5, 33),
            function: "truncate".to_owned(),
            name: "new_length".to_owned(),
            position: 2,
            found: Type::integer_unsigned(None, crate::BITLENGTH_BYTE).to_string(),
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

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

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionTypeError::ArgumentNotEvaluable {
            location: Location::new(9, 25),
            function: "another".to_owned(),
            position: 1,
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

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

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionTypeError::ReturnType {
            location: Location::new(3, 5),
            function: "another".to_owned(),
            expected: Type::boolean(None).to_string(),
            found: Type::integer_unsigned(None, crate::BITLENGTH_BYTE).to_string(),
            reference: Location::new(2, 17),
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

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

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionTypeError::NonCallable {
            location: Location::new(5, 17),
            name: Element::Type(Type::tuple(
                Some(Location::new(5, 17)),
                vec![Type::integer_unsigned(None, crate::BITLENGTH_BYTE); 2],
            ))
            .to_string(),
        }),
    ))));

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

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionTypeError::FunctionMethodSelfNotFirst {
            location: Location::new(7, 8),
            function: "method".to_owned(),
            position: 2,
            reference: Location::new(7, 26),
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
