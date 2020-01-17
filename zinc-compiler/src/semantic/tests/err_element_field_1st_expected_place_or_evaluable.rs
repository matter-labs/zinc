//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use crate::lexical::Location;

use crate::semantic::Constant;
use crate::semantic::Element;
use crate::semantic::ElementError;
use crate::semantic::Error as SemanticError;
use crate::semantic::IntegerConstant;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main(input: (), witness: ()) {
    5.data;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 6),
        ElementError::OperatorFieldFirstOperandExpectedPlaceOrEvaluable(
            Element::Constant(Constant::Integer(IntegerConstant::new(
                BigInt::from(5),
                false,
                8,
            )))
            .to_string(),
        ),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
