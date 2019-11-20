//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::Analyzer;
use crate::semantic::Error as SemanticError;
use crate::syntax::Parser;
use crate::syntax::TypeVariant;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    if 42 { 1 } else { 2 }
}
"#;

    let expected: Result<Vec<u8>, Error> = Err(Error::Semantic(
        SemanticError::ConditionalExpectedBooleanExpression(
            Location::new(3, 8),
            TypeVariant::new_integer_unsigned(8),
        ),
    ));

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
