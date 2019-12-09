//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::BinaryAnalyzer;
use crate::semantic::Element;
use crate::semantic::Error as SemanticError;
use crate::semantic::Type;
use crate::semantic::Value;
use crate::syntax::Parser;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let variable = 42;
    const CONSTANT: u8 = variable;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ExpressionIsNotConstant(
        Location::new(4, 26),
        Element::Value(Value::new(Type::new_integer_unsigned(8))),
    )));

    let result = BinaryAnalyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect(super::PANIC_SYNTAX_ERROR),
    );

    assert_eq!(expected, result);
}
