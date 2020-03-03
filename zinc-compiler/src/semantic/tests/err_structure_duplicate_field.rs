//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::Error as SemanticError;
use crate::Error;

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

    let expected = Err(Error::Semantic(SemanticError::StructureDuplicateField(
        Location::new(5, 5),
        "Data".to_owned(),
        "b".to_owned(),
    )));

    let result = super::compile_entry_point(input);

    assert_eq!(result, expected);
}
