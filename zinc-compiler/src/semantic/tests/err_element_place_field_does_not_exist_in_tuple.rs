//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::place::error::Error as PlaceError;
use crate::semantic::element::r#type::Type;
use crate::semantic::Error as SemanticError;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let tuple = (1, 2, 3);
    let result = tuple.5;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 23),
        ElementError::Place(PlaceError::FieldDoesNotExistInTuple(
            5,
            Type::tuple(vec![Type::integer_unsigned(crate::BITLENGTH_BYTE); 3]).to_string(),
        )),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(result, expected);
}
