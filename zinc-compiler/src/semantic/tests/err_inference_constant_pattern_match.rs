//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::Error as SemanticError;
use crate::semantic::IntegerConstantError;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main(input: (), witness: ()) {
    let scrutinee = 42;
    let result = match scrutinee {
        0xffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff => 10,
        2 => 20,
        _ => 30,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::InferenceConstant(
        Location::new(5, 9),
        IntegerConstantError::IntegerTooLargeForField(
            "115792089237316195423570985008687907853269984665640564039457584007913129639935"
                .to_owned(),
            crate::BITLENGTH_FIELD,
        ),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
