//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::Error as SemanticError;
use crate::semantic::StructureValueError;

use crate::Error;

#[test]
fn test() {
    let input = r#"
struct Data {
    a: u8,
}

fn main() {
    let result = struct Data {
        b: 0,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::LiteralStructure(
        Location::new(8, 9),
        StructureValueError::FieldDoesNotExist("b".to_owned(), "Data".to_owned()),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
