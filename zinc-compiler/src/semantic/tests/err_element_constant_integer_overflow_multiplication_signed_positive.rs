//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

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
    let value = 100 as i8 * 2 as i8;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowMultiplication(
                BigInt::from(200),
                Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

    let result = super::compile_entry_point(input);

    assert_eq!(result, expected);
}
