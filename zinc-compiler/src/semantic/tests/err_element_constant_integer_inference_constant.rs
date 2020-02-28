//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::integer::error::Error as IntegerConstantError;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::Error as SemanticError;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let invalid = 0xffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 19),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::IntegerTooLarge(
                "115792089237316195423570985008687907853269984665640564039457584007913129639935"
                    .to_owned(),
                crate::BITLENGTH_FIELD,
            ),
        )),
    )));

    let result = super::compile_entry_point(input);

    assert_eq!(result, expected);
}
