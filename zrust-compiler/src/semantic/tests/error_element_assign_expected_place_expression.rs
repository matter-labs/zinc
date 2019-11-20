//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use crate::lexical::Location;
use crate::semantic::Analyzer;
use crate::semantic::Element;
use crate::semantic::ElementError;
use crate::semantic::Error as SemanticError;
use crate::semantic::Integer;
use crate::semantic::Value;
use crate::syntax::Parser;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    5 = 5;
}
"#;

    let expected: Result<Vec<u8>, Error> = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 7),
        ElementError::ExpectedPlaceExpression(
            "assign",
            Element::Value(Value::Integer(Integer::new_constant(
                BigInt::from(5),
                false,
                8,
            ))),
        ),
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
