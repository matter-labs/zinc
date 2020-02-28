//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use crate::lexical::Location;

use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::Element;
use crate::semantic::Error as SemanticError;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
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

    let result = super::compile_entry_point(input);

    assert_eq!(result, expected);
}
