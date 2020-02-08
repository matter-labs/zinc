//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::caster::error::Error as CasterError;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::Error as SemanticError;
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
        ElementError::Value(ValueError::Casting(CasterError::ToLesserBitlength(128, 64))),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
