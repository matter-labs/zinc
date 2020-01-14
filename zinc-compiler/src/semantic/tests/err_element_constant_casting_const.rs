//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::CasterError;
use crate::semantic::ConstantError;
use crate::semantic::ElementError;
use crate::semantic::Error as SemanticError;
use crate::semantic::Type;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    const VALUE: u8 = 42;
    const RESULT: bool = VALUE;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 19),
        ElementError::Constant(ConstantError::Casting(CasterError::ToInvalidType(
            Type::new_integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            Type::new_boolean().to_string(),
        ))),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
