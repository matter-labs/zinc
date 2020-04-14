//!
//! The function tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::error::Error as TypeError;
use crate::semantic::element::r#type::function::error::Error as FunctionTypeError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Error as ElementError;
use crate::semantic::error::Error as SemanticError;

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(7, 24),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "another".to_owned(),
            1,
            0,
        ))),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(7, 24),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "another".to_owned(),
            "x".to_owned(),
            1,
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            Type::boolean().to_string(),
        ))),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Type(TypeError::Function(
            FunctionTypeError::argument_constantness(
                "truncate".to_owned(),
                "new_length".to_owned(),
                2,
                Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(9, 24),
        ElementError::Type(TypeError::Function(
            FunctionTypeError::argument_not_evaluable(
                "another".to_owned(),
                1,
                Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 5),
        ElementError::Type(TypeError::Function(FunctionTypeError::return_type(
            "another".to_owned(),
            Type::boolean().to_string(),
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            Location::new(2, 17),
        ))),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 24),
        ElementError::Type(TypeError::Function(FunctionTypeError::non_callable(
            Type::tuple(vec![Type::integer_unsigned(crate::BITLENGTH_BYTE); 2]).to_string(),
        ))),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(7, 8),
        ElementError::Type(TypeError::Function(
            FunctionTypeError::function_method_self_not_first(
                "method".to_owned(),
                2,
                Location::new(7, 26),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
