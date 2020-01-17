//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::ElementError;
use crate::semantic::Error as SemanticError;
use crate::semantic::IntegerValueError;
use crate::semantic::ValueError;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main(input: (), witness: ()) {
    let integer: field = 42;
    let result = -integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 18),
        ElementError::Value(ValueError::Integer(IntegerValueError::FieldNegation)),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
