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
fn main() {
    let invalid = 0xffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::InferenceConstant(
        Location::new(3, 19),
        IntegerConstantError::LiteralTooLargeForField(
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned(),
            crate::BITLENGTH_FIELD,
        ),
    )));

    let result = super::result(input);

    assert_eq!(expected, result);
}
