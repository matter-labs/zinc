//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::integer::error::Error as IntegerConstantError;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::Error as SemanticError;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    42 as u64 ..= 69 as u128
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 15),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchRangeInclusive(
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
