//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::BinaryAnalyzer;
use crate::semantic::CasterError;
use crate::semantic::ConstantError;
use crate::semantic::ElementError;
use crate::semantic::Error as SemanticError;
use crate::semantic::Type;
use crate::syntax::Parser;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let value: bool = 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 5),
        ElementError::Constant(ConstantError::Casting(CasterError::ToInvalidType(
            Type::new_integer_unsigned(8),
            Type::new_boolean(),
        ))),
    )));

    let result = BinaryAnalyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect(super::PANIC_SYNTAX_ERROR),
    );

    assert_eq!(expected, result);
}
