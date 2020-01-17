//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::Element;
use crate::semantic::ElementError;
use crate::semantic::Error as SemanticError;
use crate::semantic::Type;

use crate::Error;

#[test]
fn test() {
    let input = r#"
type X = u8;

fn main(input: (), witness: ()) {
    let value = 42 / X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 20),
        ElementError::OperatorDivisionSecondOperandExpectedEvaluable(
            Element::Type(Type::new_integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
