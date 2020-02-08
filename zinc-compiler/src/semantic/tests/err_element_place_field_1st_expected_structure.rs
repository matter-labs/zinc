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
    let tuple = (true, true, false);
    let value = tuple.first;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 22),
        ElementError::Place(PlaceError::OperatorFieldFirstOperandExpectedStructure(
            Type::tuple(vec![Type::boolean(); 3]).to_string(),
        )),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
