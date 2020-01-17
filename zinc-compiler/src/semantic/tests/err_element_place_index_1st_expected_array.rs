//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::ElementError;
use crate::semantic::Error as SemanticError;
use crate::semantic::PlaceError;
use crate::semantic::Type;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main(input: (), witness: ()) {
    let tuple = (true, false, true);
    let value = tuple[1];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 22),
        ElementError::Place(PlaceError::OperatorIndexFirstOperandExpectedArray(
            Type::new_tuple(vec![Type::new_boolean(); 3]).to_string(),
        )),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
