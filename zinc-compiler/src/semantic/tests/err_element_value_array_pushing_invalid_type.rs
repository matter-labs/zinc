//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::ArrayValueError;

use crate::semantic::Error as SemanticError;
use crate::semantic::Type;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let array = [1, false];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::LiteralArray(
        Location::new(3, 21),
        ArrayValueError::PushingInvalidType(
            Type::new_boolean().to_string(),
            Type::new_integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        ),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
