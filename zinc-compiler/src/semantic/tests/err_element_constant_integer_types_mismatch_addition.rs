//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::ConstantError;
use crate::semantic::ElementError;
use crate::semantic::Error as SemanticError;
use crate::semantic::IntegerConstantError;
use crate::semantic::Type;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let value = 42 as u64 + 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchAddition(
                Type::new_integer_unsigned(64).to_string(),
                Type::new_integer_unsigned(128).to_string(),
            ),
        )),
    )));

    let result = super::result(input);

    assert_eq!(expected, result);
}
