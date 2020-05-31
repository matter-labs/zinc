//!
//! The structure value element tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::element::value::structure::error::Error as StructureValueError;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_field_does_not_exist() {
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
        ElementError::Value(ValueError::Structure(
            StructureValueError::FieldDoesNotExist {
                location: Location::new(9, 7),
                type_identifier: "Data".to_owned(),
                field_name: "b".to_owned(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_field_expected() {
    let input = r#"
struct Data {
    a: u8,
    b: u8,
}

fn main() {
    let result = Data {
        a: 42,
        c: 64,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::Structure(StructureValueError::FieldExpected {
            location: Location::new(10, 9),
            type_identifier: "Data".to_owned(),
            position: 2,
            expected: "b".to_owned(),
            found: "c".to_owned(),
        })),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_field_invalid_type() {
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
        ElementError::Value(ValueError::Structure(
            StructureValueError::FieldInvalidType {
                location: Location::new(8, 9),
                type_identifier: "Data".to_owned(),
                field_name: "a".to_owned(),
                expected: Type::integer_unsigned(None, zinc_const::BITLENGTH_BYTE).to_string(),
                found: Type::boolean(None).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_field_out_of_range() {
    let input = r#"
struct Data {
    a: u8,
    b: u8,
}

fn main() {
    let result = Data {
        a: 42,
        b: 25,
        c: 64,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::Structure(
            StructureValueError::FieldOutOfRange {
                location: Location::new(11, 9),
                type_identifier: "Data".to_owned(),
                expected: 2,
                found: 3,
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
