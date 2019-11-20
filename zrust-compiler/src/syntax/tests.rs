//!
//! The syntax parser semantic.tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Location;
use crate::syntax::BlockExpression;
use crate::syntax::CircuitProgram;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Field;
use crate::syntax::FnStatement;
use crate::syntax::Identifier;
use crate::syntax::OuterStatement;
use crate::syntax::Parser;
use crate::syntax::Type;
use crate::syntax::TypeVariant;

#[test]
fn ok() {
    let input = r#"fn f(a: field) {}"#;

    let expected = Ok(CircuitProgram {
        statements: vec![OuterStatement::Fn(FnStatement::new(
            Location::new(1, 1),
            Identifier::new(Location::new(1, 4), "f".to_owned()),
            vec![Field::new(
                Location::new(1, 6),
                Identifier::new(Location::new(1, 6), "a".to_owned()),
                Type::new(Location::new(1, 9), TypeVariant::new_field()),
            )],
            Type::new(Location::new(1, 1), TypeVariant::new_unit()),
            BlockExpression::new(Location::new(1, 16), vec![], None),
        ))],
    });

    let result = Parser::default().parse(input.to_owned());

    assert_eq!(expected, result);
}

#[test]
fn error_expected() {
    let input = "let";

    let result: Result<CircuitProgram, Error> = Parser::default().parse(input.to_owned());

    let expected: Result<CircuitProgram, Error> = Err(Error::Syntax(SyntaxError::Expected(
        Location::new(1, 1),
        vec!["type", "struct", "enum", "fn", "mod", "use"],
        Lexeme::Keyword(Keyword::Let),
    )));

    assert_eq!(expected, result);
}

#[test]
fn error_unexpected_eof() {
    let input = "fn";

    let result: Result<CircuitProgram, Error> = Parser::default().parse(input.to_owned());

    let expected: Result<CircuitProgram, Error> = Err(Error::Syntax(SyntaxError::Expected(
        Location::new(1, 3),
        vec!["{identifier}"],
        Lexeme::Eof,
    )));

    assert_eq!(expected, result);
}
