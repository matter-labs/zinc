//!
//! The type statement parser.
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
use crate::syntax::TypeParser;
use crate::syntax::TypeStatement;
use crate::syntax::TypeStatementBuilder;

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordType,
    Identifier,
    Equals,
    Type,
    Semicolon,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordType
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    next: Option<Token>,
    builder: TypeStatementBuilder,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(TypeStatement, Option<Token>), Error> {
        loop {
            match self.state {
                State::KeywordType => {
                    match crate::syntax::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Type),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::Identifier;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["type"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::Identifier => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            let identifier = Identifier::new(location, identifier.name);
                            self.builder.set_identifier(identifier);
                            self.state = State::Equals;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{identifier}"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::Equals => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Equals),
                            ..
                        } => self.state = State::Type,
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["="],
                                lexeme,
                            )));
                        }
                    }
                }
                State::Type => {
                    let (r#type, next) = TypeParser::default().parse(stream.clone(), None)?;
                    self.next = next;
                    self.builder.set_type(r#type);
                    self.state = State::Semicolon;
                }
                State::Semicolon => match crate::syntax::take_or_next(self.next.take(), stream)? {
                    Token {
                        lexeme: Lexeme::Symbol(Symbol::Semicolon),
                        ..
                    } => return Ok((self.builder.finish(), None)),
                    Token { lexeme, location } => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            vec![";"],
                            lexeme,
                        )));
                    }
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::error::Error;
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Symbol;
    use crate::lexical::TokenStream;
    use crate::syntax::Error as SyntaxError;
    use crate::syntax::Identifier;
    use crate::syntax::Type;
    use crate::syntax::TypeStatement;
    use crate::syntax::TypeVariant;

    #[test]
    fn ok() {
        let input = r#"type X = field;"#;

        let expected = Ok((
            TypeStatement::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 6), "X".to_owned()),
                Type::new(Location::new(1, 10), TypeVariant::Field),
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn err_no_value() {
        let input = r#"type X;"#;

        let expected = Err(Error::Syntax(SyntaxError::Expected(
            Location::new(1, 7),
            vec!["="],
            Lexeme::Symbol(Symbol::Semicolon),
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}