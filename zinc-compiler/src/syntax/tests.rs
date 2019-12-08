//!
//! The syntax parser semantic.tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Location;
use crate::syntax::CircuitProgram;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Parser;

#[test]
fn err_expected() {
    let input = "bool";

    let result: Result<CircuitProgram, Error> = Parser::default().parse(input.to_owned());

    let expected: Result<CircuitProgram, Error> = Err(Error::Syntax(SyntaxError::Expected(
        Location::new(1, 1),
        vec![
            "const", "static", "type", "struct", "enum", "fn", "mod", "use",
        ],
        Lexeme::Keyword(Keyword::Bool),
    )));

    assert_eq!(expected, result);
}

#[test]
fn err_expected_not_eof() {
    let input = "fn";

    let result: Result<CircuitProgram, Error> = Parser::default().parse(input.to_owned());

    let expected: Result<CircuitProgram, Error> = Err(Error::Syntax(SyntaxError::Expected(
        Location::new(1, 3),
        vec!["{identifier}"],
        Lexeme::Eof,
    )));

    assert_eq!(expected, result);
}
