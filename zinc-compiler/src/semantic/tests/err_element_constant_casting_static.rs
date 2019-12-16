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
static VALUE: u8 = 42;
static RESULT: bool = VALUE;

fn main() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 16),
        ElementError::Constant(ConstantError::Casting(CasterError::ToInvalidType(
            Type::new_integer_unsigned(8).to_string(),
            Type::new_boolean().to_string(),
        ))),
    )));

    let result = super::result(input);

    assert_eq!(expected, result);
}
