//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::BinaryAnalyzer;
use crate::semantic::Element;
use crate::semantic::ElementError;
use crate::semantic::Error as SemanticError;
use crate::semantic::Type;
use crate::syntax::Parser;
use crate::Error;

#[test]
fn test() {
    let input = r#"
type X = u8;

fn main() {
    let mut value = 0;
    value = X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(6, 11),
        ElementError::OperatorAssignmentSecondOperandExpectedEvaluable(Element::Type(
            Type::new_integer_unsigned(8),
        )),
    )));

    let result = BinaryAnalyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect(super::PANIC_SYNTAX_ERROR),
    );

    assert_eq!(expected, result);
}
