//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::Analyzer;
use crate::semantic::CastingError;
use crate::semantic::ElementError;
use crate::semantic::Error as SemanticError;
use crate::semantic::IntegerError;
use crate::semantic::ValueError;
use crate::syntax::Parser;
use crate::syntax::TypeVariant;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let value: bool = 42;
}
"#;

    let expected: Result<Vec<u8>, Error> = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 5),
        ElementError::Value(ValueError::Integer(IntegerError::Casting(
            CastingError::ToInvalidType(
                TypeVariant::new_integer_unsigned(8),
                TypeVariant::new_boolean(),
            ),
        ))),
    )));

    let result = Analyzer::default()
        .compile(
            Parser::default()
                .parse(input.to_owned())
                .expect("Syntax error"),
        )
        .map(|instructions| {
            instructions
                .into_iter()
                .map(|instruction| instruction.encode())
                .flatten()
                .collect::<Vec<u8>>()
        });

    assert_eq!(expected, result);
}
