//!
//! The outer statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::ConstStatementParser;
use crate::syntax::EnumStatementParser;
use crate::syntax::Error as SyntaxError;
use crate::syntax::FnStatementParser;
use crate::syntax::ModStatementParser;
use crate::syntax::OuterStatement;
use crate::syntax::StaticStatementParser;
use crate::syntax::StructStatementParser;
use crate::syntax::TypeStatementParser;
use crate::syntax::UseStatementParser;

#[derive(Default)]
pub struct Parser {}

impl Parser {
    pub fn parse(
        self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(OuterStatement, Option<Token>), Error> {
        match match initial.take() {
            Some(token) => token,
            None => stream.borrow_mut().next()?,
        } {
            token @ Token {
                lexeme: Lexeme::Keyword(Keyword::Const),
                ..
            } => ConstStatementParser::default()
                .parse(stream.clone(), Some(token))
                .map(|(statement, next)| (OuterStatement::Const(statement), next)),
            token @ Token {
                lexeme: Lexeme::Keyword(Keyword::Static),
                ..
            } => StaticStatementParser::default()
                .parse(stream.clone(), Some(token))
                .map(|(statement, next)| (OuterStatement::Static(statement), next)),
            token @ Token {
                lexeme: Lexeme::Keyword(Keyword::Type),
                ..
            } => TypeStatementParser::default()
                .parse(stream.clone(), Some(token))
                .map(|(statement, next)| (OuterStatement::Type(statement), next)),
            token @ Token {
                lexeme: Lexeme::Keyword(Keyword::Struct),
                ..
            } => StructStatementParser::default()
                .parse(stream.clone(), Some(token))
                .map(|(statement, next)| (OuterStatement::Struct(statement), next)),
            token @ Token {
                lexeme: Lexeme::Keyword(Keyword::Enum),
                ..
            } => EnumStatementParser::default()
                .parse(stream.clone(), Some(token))
                .map(|(statement, next)| (OuterStatement::Enum(statement), next)),
            token @ Token {
                lexeme: Lexeme::Keyword(Keyword::Fn),
                ..
            } => FnStatementParser::default()
                .parse(stream.clone(), Some(token))
                .map(|(statement, next)| (OuterStatement::Fn(statement), next)),
            token @ Token {
                lexeme: Lexeme::Keyword(Keyword::Mod),
                ..
            } => ModStatementParser::default()
                .parse(stream.clone(), Some(token))
                .map(|(statement, next)| (OuterStatement::Mod(statement), next)),
            token @ Token {
                lexeme: Lexeme::Keyword(Keyword::Use),
                ..
            } => UseStatementParser::default()
                .parse(stream.clone(), Some(token))
                .map(|(statement, next)| (OuterStatement::Use(statement), next)),
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::Expected(
                location,
                vec![
                    "const", "static", "type", "struct", "enum", "fn", "mod", "use",
                ],
                lexeme,
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::lexical::Location;
    use crate::lexical::TokenStream;
    use crate::syntax::BlockExpression;
    use crate::syntax::Field;
    use crate::syntax::FnStatement;
    use crate::syntax::Identifier;
    use crate::syntax::OuterStatement;
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;

    #[test]
    fn ok() {
        let input = r#"fn f(a: field) {}"#;

        let expected = Ok((
            OuterStatement::Fn(FnStatement::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 4), "f".to_owned()),
                vec![Field::new(
                    Location::new(1, 6),
                    Identifier::new(Location::new(1, 6), "a".to_owned()),
                    Type::new(Location::new(1, 9), TypeVariant::new_field()),
                )],
                Type::new(Location::new(1, 1), TypeVariant::new_unit()),
                BlockExpression::new(Location::new(1, 16), vec![], None),
            )),
            None,
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }
}
