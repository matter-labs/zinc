//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::caster::error::Error as CasterError;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::Error as SemanticError;

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
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            Type::boolean().to_string(),
        ))),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(result, expected);
}
