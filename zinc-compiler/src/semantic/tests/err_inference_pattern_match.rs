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
    let scrutinee = 42;
    let result = match scrutinee {
        0xffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff => 10,
        2 => 20,
        _ => 30,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::InferencePatternMatch(
        Location::new(5, 9),
        IntegerConstantError::LiteralTooLargeForField(
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned(),
            crate::BITLENGTH_FIELD,
        ),
    )));

    let result = super::result(input);

    assert_eq!(expected, result);
}
