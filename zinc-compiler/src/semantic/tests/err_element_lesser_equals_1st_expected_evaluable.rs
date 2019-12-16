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

fn main() {
    let value = X <= 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 19),
        ElementError::OperatorLesserEqualsFirstOperandExpectedEvaluable(
            Element::Type(Type::new_integer_unsigned(8)).to_string(),
        ),
    )));

    let result = super::result(input);

    assert_eq!(expected, result);
}
