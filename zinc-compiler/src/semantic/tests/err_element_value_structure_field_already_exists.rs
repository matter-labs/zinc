//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::element::value::structure::error::Error as StructureValueError;
use crate::semantic::Error as SemanticError;

use crate::Error;

#[test]
fn test() {
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

    let expected = Err(Error::Semantic(SemanticError::LiteralStructure(
        Location::new(9, 9),
        StructureValueError::FieldAlreadyExists("a".to_owned(), "Data".to_owned()),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(result, expected);
}
