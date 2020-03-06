//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::Location;
use crate::semantic::Error as SemanticError;

#[test]
fn test() {
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

    let expected = Err(Error::Semantic(SemanticError::StructureDuplicateField {
        location: Location::new(5, 5),
        type_identifier: "Data".to_owned(),
        field_name: "b".to_owned(),
    }));

    let result = super::compile_entry_point(input);

    assert_eq!(result, expected);
}
