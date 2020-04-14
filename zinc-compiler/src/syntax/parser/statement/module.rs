//!
//! The mod statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::stream::TokenStream;
use crate::lexical::token::lexeme::keyword::Keyword;
use crate::lexical::token::lexeme::symbol::Symbol;
use crate::lexical::token::lexeme::Lexeme;
use crate::lexical::token::Token;
use crate::syntax::error::Error as SyntaxError;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::statement::module::builder::Builder as ModStatementBuilder;
use crate::syntax::tree::statement::module::Statement as ModStatement;

#[derive(Default)]
pub struct Parser {
    builder: ModStatementBuilder,
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses a 'mod' statement.
    ///
    /// 'mod jabberwocky;'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(ModStatement, Option<Token>), Error> {
        match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
            Token {
                lexeme: Lexeme::Keyword(Keyword::Mod),
                location,
            } => {
                self.builder.set_location(location);
            }
            Token { lexeme, location } => {
                return Err(Error::Syntax(SyntaxError::expected_one_of(
                    location,
                    vec!["mod"],
                    lexeme,
                    None,
                )));
            }
        }

        match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
            Token {
                lexeme: Lexeme::Identifier(identifier),
                location,
            } => {
                let identifier = Identifier::new(location, identifier.inner);
                self.builder.set_identifier(identifier);
            }
            Token { lexeme, location } => {
                return Err(Error::Syntax(SyntaxError::expected_identifier(
                    location, lexeme, None,
                )))
            }
        }

        match crate::syntax::parser::take_or_next(self.next.take(), stream)? {
            Token {
                lexeme: Lexeme::Symbol(Symbol::Semicolon),
                ..
            } => Ok((self.builder.finish(), None)),
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::expected_one_of(
                location,
                vec![";"],
                lexeme,
                None,
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::error::Error;
    use crate::lexical::stream::TokenStream;
    use crate::lexical::token::lexeme::symbol::Symbol;
    use crate::lexical::token::lexeme::Lexeme;
    use crate::lexical::token::location::Location;
    use crate::syntax::error::Error as SyntaxError;
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::statement::module::Statement as ModStatement;

    #[test]
    fn ok() {
        let input = r#"mod jabberwocky;"#;

        let expected = Ok((
            ModStatement::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 5), "jabberwocky".to_owned()),
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_identifier() {
        let input = r#"mod;"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_identifier(
            Location::new(1, 4),
            Lexeme::Symbol(Symbol::Semicolon),
            None,
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_semicolon() {
        let input = r#"mod jabberwocky"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_one_of(
            Location::new(1, 16),
            vec![";"],
            Lexeme::Eof,
            None,
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
