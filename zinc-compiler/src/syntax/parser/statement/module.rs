//!
//! The mod statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Identifier;
use crate::syntax::ModStatement;
use crate::syntax::ModStatementBuilder;

#[derive(Default)]
pub struct Parser {
    builder: ModStatementBuilder,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<ModStatement, Error> {
        match match initial.take() {
            Some(token) => token,
            None => stream.borrow_mut().next()?,
        } {
            Token {
                lexeme: Lexeme::Keyword(Keyword::Mod),
                location,
            } => {
                self.builder.set_location(location);
            }
            Token { lexeme, location } => {
                return Err(Error::Syntax(SyntaxError::Expected(
                    location,
                    vec!["mod"],
                    lexeme,
                )));
            }
        }

        let next = stream.borrow_mut().next()?;
        match next {
            Token {
                lexeme: Lexeme::Identifier(identifier),
                location,
            } => {
                let identifier = Identifier::new(location, identifier.name);
                self.builder.set_identifier(identifier);
            }
            Token { lexeme, location } => {
                return Err(Error::Syntax(SyntaxError::Expected(
                    location,
                    vec!["{identifier}"],
                    lexeme,
                )))
            }
        }

        let next = stream.borrow_mut().next()?;
        match next {
            Token {
                lexeme: Lexeme::Symbol(Symbol::Semicolon),
                ..
            } => Ok(self.builder.finish()),
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::Expected(
                location,
                vec![";"],
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
    use crate::syntax::Identifier;
    use crate::syntax::ModStatement;

    #[test]
    fn ok() {
        let input = r#"mod jabberwocky;"#;

        let expected = Ok(ModStatement::new(
            Location::new(1, 1),
            Identifier::new(Location::new(1, 5), "jabberwocky".to_owned()),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }
}
