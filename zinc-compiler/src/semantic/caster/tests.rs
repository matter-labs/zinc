//!
//! The type caster tests.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::caster::error::Error as CasterError;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::Error as SemanticError;
use crate::Error;

#[test]
fn from_invalid_type() {
    let input = r#"
fn main() {
    let value: field = 0;
    let result = value as u8;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 24),
        ElementError::Value(ValueError::Casting(CasterError::casting_from_invalid_type(
            &Type::field(),
            &Type::integer_unsigned(crate::BITLENGTH_BYTE),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn to_invalid_type() {
    let input = r#"
fn main() {
    let value: u8 = 0;
    let result = value as bool;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 24),
        ElementError::Value(ValueError::Casting(CasterError::casting_to_invalid_type(
            &Type::integer_unsigned(crate::BITLENGTH_BYTE),
            &Type::boolean(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn to_invalid_type_let_implicit() {
    let input = r#"
fn main() {
    let value = 42;
    let result: bool = value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 17),
        ElementError::Value(ValueError::Casting(CasterError::casting_to_invalid_type(
            &Type::integer_unsigned(crate::BITLENGTH_BYTE),
            &Type::boolean(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn to_lesser_bitlength() {
    let input = r#"
fn main() {
    let value: u128 = 0;
    let result = value as u64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 24),
        ElementError::Value(ValueError::Casting(
            CasterError::casting_integer_to_lesser_bitlength(128, 64),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}
