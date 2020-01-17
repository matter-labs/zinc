//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::ConstantError;
use crate::semantic::ElementError;
use crate::semantic::Error as SemanticError;
use crate::semantic::IntegerConstantError;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main(input: (), witness: ()) {
    let value = -(42 as field);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 17),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::NegationBitlengthTooBig(crate::BITLENGTH_FIELD),
        )),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
