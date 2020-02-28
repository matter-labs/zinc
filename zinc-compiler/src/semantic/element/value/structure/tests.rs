//!
//! The structure tests.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::element::value::structure::error::Error as StructureValueError;
use crate::semantic::Error as SemanticError;
use crate::Error;

#[test]
fn field_already_exists() {
    let input = r#"
struct Data {
    a: u8,
}

fn main() {
    let result = Data {
        a: 0,
        a: 1,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(9, 9),
        ElementError::Value(ValueError::Structure(
            StructureValueError::FieldAlreadyExists("a".to_owned(), "Data".to_owned()),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn field_does_not_exist() {
    let input = r#"
struct Data {
    a: u8,
}

fn main() {
    let result = Data {
        a: 0,
    }.b;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(9, 6),
        ElementError::Value(ValueError::Structure(
            StructureValueError::FieldDoesNotExist("b".to_owned(), "Data".to_owned()),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn field_invalid_type() {
    let input = r#"
struct Data {
    a: u8,
}

fn main() {
    let result = Data {
        a: true,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(8, 12),
        ElementError::Value(ValueError::Structure(
            StructureValueError::FieldInvalidType(
                "a".to_owned(),
                "Data".to_owned(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
                Type::boolean().to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}
