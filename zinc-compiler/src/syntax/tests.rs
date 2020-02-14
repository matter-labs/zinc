//!
//! The syntax parser semantic.tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Location;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Parser;
use crate::syntax::SyntaxTree;

#[test]
fn err_expected() {
    let input = "bool";

    let result: Result<SyntaxTree, Error> = Parser::default().parse(input);

    let expected: Result<SyntaxTree, Error> = Err(Error::Syntax(SyntaxError::Expected(
        Location::new(1, 1),
        vec![
            "const", "static", "type", "struct", "enum", "fn", "mod", "use", "impl",
        ],
        Lexeme::Keyword(Keyword::Bool),
    )));

    assert_eq!(result, expected);
}

#[test]
fn err_expected_not_eof() {
    let input = "fn";

    let result: Result<SyntaxTree, Error> = Parser::default().parse(input);

    let expected: Result<SyntaxTree, Error> = Err(Error::Syntax(SyntaxError::Expected(
        Location::new(1, 3),
        vec!["{identifier}"],
        Lexeme::Eof,
    )));

    assert_eq!(result, expected);
}
