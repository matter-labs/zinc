//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::place::error::Error as PlaceError;
use crate::semantic::Error as SemanticError;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let array = [1, 2, 3, 4, 5];
    let slice = array[0 .. 6];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 22),
        ElementError::Place(PlaceError::IndexSliceEndOutOfRange(
            "6".to_owned(),
            "5".to_owned(),
        )),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
