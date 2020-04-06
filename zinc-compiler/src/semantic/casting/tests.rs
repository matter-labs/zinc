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
