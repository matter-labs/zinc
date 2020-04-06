//!
//! The semantic analyzer structure type element tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::error::Error as TypeError;
use crate::semantic::element::r#type::structure::error::Error as StructureTypeError;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_duplicate_field() {
    let input = r#"
struct Data {
    a: u8,
    b: u8,
    b: field,
}

fn main() -> u8 {
    42
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 5),
        ElementError::Type(TypeError::Structure(StructureTypeError::DuplicateField {
            type_identifier: "Data".to_owned(),
            field_name: "b".to_owned(),
        })),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
