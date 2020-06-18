//!
//! The attribute parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::stream::TokenStream;
use crate::lexical::token::lexeme::symbol::Symbol;
use crate::lexical::token::lexeme::Lexeme;
use crate::lexical::token::Token;
use crate::syntax::error::Error as SyntaxError;
use crate::syntax::tree::attribute::builder::Builder as AttributeBuilder;
use crate::syntax::tree::attribute::Attribute;
use crate::syntax::tree::identifier::Identifier;

#[derive(Debug, Clone, Copy)]
pub enum State {
    NumberSign,
    ExclamationMarkOrNext,
    BrackerSquareLeft,
    Identifier,
    BrackerSquareRight,
}

impl Default for State {
    fn default() -> Self {
        Self::NumberSign
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: AttributeBuilder,
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses an attribute.
    ///
    /// '#[test]'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(Attribute, Option<Token>), Error> {
        loop {
            match self.state {
                State::NumberSign => {
                    match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Number),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::ExclamationMarkOrNext;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["#"],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::ExclamationMarkOrNext => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ExclamationMark),
                            ..
                        } => {
                            self.builder.set_inner();
                        }
                        token => {
                            self.next = Some(token);
                        }
                    }

                    self.state = State::BrackerSquareLeft;
                }
                State::BrackerSquareLeft => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareLeft),
                            ..
                        } => self.state = State::Identifier,
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["["],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::Identifier => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            let identifier = Identifier::new(location, identifier.inner);
                            self.builder.set_identifier(identifier);
                            self.state = State::BrackerSquareRight;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_identifier(
                                location, lexeme, None,
                            )));
                        }
                    }
                }
                State::BrackerSquareRight => {
                    return match crate::syntax::parser::take_or_next(
                        self.next.take(),
                        stream.clone(),
                    )? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        } => Ok((self.builder.finish(), self.next.take())),
                        Token { lexeme, location } => Err(Error::Syntax(
                            SyntaxError::expected_one_of(location, vec!["]"], lexeme, None),
                        )),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::error::Error;
    use crate::lexical::stream::TokenStream;
    use crate::lexical::token::lexeme::symbol::Symbol;
    use crate::lexical::token::lexeme::Lexeme;
    use crate::lexical::token::location::Location;
    use crate::syntax::error::Error as SyntaxError;
    use crate::syntax::tree::attribute::Attribute;
    use crate::syntax::tree::identifier::Identifier;

    #[test]
    fn ok_outer() {
        let input = r#"#[test]"#;

        let expected = Ok((
            Attribute::new(
                Location::new(1, 1),
                false,
                Identifier::new(Location::new(1, 3), "test".to_owned()),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_inner() {
        let input = r#"#![test]"#;

        let expected = Ok((
            Attribute::new(
                Location::new(1, 1),
                true,
                Identifier::new(Location::new(1, 4), "test".to_owned()),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_bracket_square_left() {
        let input = r#"#(test]"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_one_of(
            Location::new(1, 2),
            vec!["["],
            Lexeme::Symbol(Symbol::ParenthesisLeft),
            None,
        )));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_identifier() {
        let input = r#"#[=]"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_identifier(
            Location::new(1, 3),
            Lexeme::Symbol(Symbol::Equals),
            None,
        )));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_bracket_square_right() {
        let input = r#"#[test)"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_one_of(
            Location::new(1, 7),
            vec!["]"],
            Lexeme::Symbol(Symbol::ParenthesisRight),
            None,
        )));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
