//!
//! The type caster tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::casting::error::Error as CastingError;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::error::Error as SemanticError;

#[test]
fn ok_integer_lesser_bitlength_same_sign() {
    let input = r#"
fn main() {
    let value: u64 = 0;
    let result = value as u32;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_integer_lesser_bitlength_different_sign() {
    let input = r#"
fn main() {
    let value: u64 = 0;
    let result = value as i32;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_integer_equal_bitlength_same_sign() {
    let input = r#"
fn main() {
    let value: u64 = 0;
    let result = value as u64;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_integer_equal_bitlength_different_sign() {
    let input = r#"
fn main() {
    let value: u64 = 0;
    let result = value as i64;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_integer_greater_bitlength_same_sign() {
    let input = r#"
fn main() {
    let value: u64 = 0;
    let result = value as u128;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_integer_greater_bitlength_different_sign() {
    let input = r#"
fn main() {
    let value: u64 = 0;
    let result = value as i128;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_integer_unsigned_to_field() {
    let input = r#"
fn main() {
    let value: u64 = 0;
    let result = value as field;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_integer_signed_to_field() {
    let input = r#"
fn main() {
    let value: i64 = 0;
    let result = value as field;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_integer_enumeration_to_unsigned() {
    let input = r#"
enum List {
    VALUE = 42,
}

fn main() {
    let value = List::VALUE;
    let result = value as u64;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_integer_enumeration_to_signed() {
    let input = r#"
enum List {
    VALUE = 42,
}

fn main() {
    let value = List::VALUE;
    let result = value as i64;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_integer_enumeration_to_field() {
    let input = r#"
enum List {
    VALUE = 42,
}

fn main() {
    let value = List::VALUE;
    let result = value as field;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_integer_field_to_field() {
    let input = r#"
fn main() {
    let value: field = 0;
    let result = value as field;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_same_type() {
    let input = r#"
struct Data {
    value: u8,
}

fn main() {
    let data = Data { value: 42 };
    let result = data as Data;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn error_casting_from_invalid_type() {
    let input = r#"
fn main() {
    let value: field = 0;
    let result = value as u8;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 24),
        ElementError::Value(ValueError::Casting(
            CastingError::casting_from_invalid_type(
                &Type::field(),
                &Type::integer_unsigned(crate::BITLENGTH_BYTE),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_casting_to_invalid_type() {
    let input = r#"
fn main() {
    let value: u8 = 0;
    let result = value as bool;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 24),
        ElementError::Value(ValueError::Casting(CastingError::casting_to_invalid_type(
            &Type::integer_unsigned(crate::BITLENGTH_BYTE),
            &Type::boolean(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_casting_to_invalid_type_let_implicit() {
    let input = r#"
fn main() {
    let value = 42;
    let result: bool = value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 17),
        ElementError::Value(ValueError::Casting(CastingError::casting_to_invalid_type(
            &Type::integer_unsigned(crate::BITLENGTH_BYTE),
            &Type::boolean(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
