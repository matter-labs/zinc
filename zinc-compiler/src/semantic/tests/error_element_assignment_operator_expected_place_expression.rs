//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use crate::lexical::Location;
use crate::semantic::BinaryAnalyzer;
use crate::semantic::Constant;
use crate::semantic::Element;
use crate::semantic::ElementError;
use crate::semantic::Error as SemanticError;
use crate::semantic::IntegerConstant;
use crate::syntax::Parser;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    5 = 5;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 7),
        ElementError::OperatorAssignmentFirstOperandExpectedPlace(Element::Constant(
            Constant::Integer(IntegerConstant::new(BigInt::from(5), false, 8)),
        )),
    )));

    let result = BinaryAnalyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect(super::PANIC_SYNTAX_ERROR),
    );

    assert_eq!(expected, result);
}
