//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::CasterError;
use crate::semantic::ElementError;
use crate::semantic::Error as SemanticError;
use crate::semantic::ValueError;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let value: u128 = 0;
    let result = value as u64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 24),
        ElementError::Value(ValueError::Casting(CasterError::DataLossPossible(128, 64))),
    )));

    let result = super::result(input);

    assert_eq!(expected, result);
}
