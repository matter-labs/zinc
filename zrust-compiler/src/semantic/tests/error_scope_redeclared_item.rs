//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::Analyzer;
use crate::semantic::Error as SemanticError;
use crate::semantic::ScopeError;
use crate::syntax::Parser;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let result = 42;
    let result = 69;
}
"#;

    let expected: Result<Vec<u8>, Error> = Err(Error::Semantic(SemanticError::Scope(
        Location::new(4, 5),
        ScopeError::RedeclaredItem("result".to_owned()),
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
