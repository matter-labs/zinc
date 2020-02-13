//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::value::error::Error as ValueError;
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
    }.b;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(9, 6),
        ElementError::Value(ValueError::Structure(
            StructureValueError::FieldDoesNotExist("b".to_owned(), "Data".to_owned()),
        )),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(result, expected);
}
