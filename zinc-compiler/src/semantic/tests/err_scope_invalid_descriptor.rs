//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::Error as SemanticError;
use crate::semantic::IntegerConstant;
use crate::semantic::PlaceDescriptor;
use crate::semantic::ScopeError;
use crate::semantic::Type;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let array = (1, 2, 3);
    let element = array[1];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Scope(
        Location::new(4, 19),
        ScopeError::InvalidDescriptor(
            Type::new_tuple(vec![
                Type::new_integer_unsigned(8),
                Type::new_integer_unsigned(8),
                Type::new_integer_unsigned(8),
            ])
            .to_string(),
            PlaceDescriptor::ArrayIndexConstant(IntegerConstant::from((1, crate::BITLENGTH_BYTE))),
        ),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
