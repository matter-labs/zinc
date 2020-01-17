//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::CasterError;
use crate::semantic::ElementError;
use crate::semantic::Error as SemanticError;
use crate::semantic::Type;
use crate::semantic::ValueError;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main(input: (), witness: ()) {
    let value: u8 = 0;
    let result = value as bool;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 24),
        ElementError::Value(ValueError::Casting(CasterError::ToInvalidType(
            Type::new_integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            Type::new_boolean().to_string(),
        ))),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
