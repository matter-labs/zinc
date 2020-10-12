//!
//! The attribute parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Lexeme;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::Error as SyntaxError;
use crate::error::ParsingError;
use crate::tree::attribute::builder::Builder as AttributeBuilder;
use crate::tree::attribute::Attribute;
use crate::tree::identifier::Identifier;

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    NumberSign,
    /// The `#` has been parsed so far.
    ExclamationMarkOrNext,
    /// The `#` with an optional `!` have been parsed so far.
    BrackerSquareLeft,
    /// The `#[` has been parsed so far.
    Identifier,
    /// The `#[ {identifier}` has been parsed so far.
    BrackerSquareRight,
}

impl Default for State {
    fn default() -> Self {
        Self::NumberSign
    }
}

///
/// The attribute parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The builder of the parsed value.
    builder: AttributeBuilder,
    /// The token returned from a subparser.
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
    ) -> Result<(Attribute, Option<Token>), ParsingError> {
        loop {
            match self.state {
                State::NumberSign => {
                    match crate::parser::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Number),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::ExclamationMarkOrNext;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["#"],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::ExclamationMarkOrNext => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
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
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareLeft),
                            ..
                        } => self.state = State::Identifier,
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["["],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::Identifier => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            let identifier = Identifier::new(location, identifier.inner);
                            self.builder.set_identifier(identifier);
                            self.state = State::BrackerSquareRight;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_identifier(
                                location, lexeme, None,
                            )));
                        }
                    }
                }
                State::BrackerSquareRight => {
                    return match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        } => Ok((self.builder.finish(), self.next.take())),
                        Token { lexeme, location } => Err(ParsingError::Syntax(
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
    use zinc_lexical::Lexeme;
    use zinc_lexical::Location;
    use zinc_lexical::Symbol;
    use zinc_lexical::TokenStream;

    use super::Parser;
    use crate::error::Error as SyntaxError;
    use crate::error::ParsingError;
    use crate::tree::attribute::Attribute;
    use crate::tree::identifier::Identifier;

    #[test]
    fn ok_outer() {
        let input = r#"#[test]"#;

        let expected = Ok((
            Attribute::new(
                Location::test(1, 1),
                false,
                Identifier::new(Location::test(1, 3), "test".to_owned()),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_inner() {
        let input = r#"#![test]"#;

        let expected = Ok((
            Attribute::new(
                Location::test(1, 1),
                true,
                Identifier::new(Location::test(1, 4), "test".to_owned()),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_bracket_square_left() {
        let input = r#"#(test]"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_one_of(
            Location::test(1, 2),
            vec!["["],
            Lexeme::Symbol(Symbol::ParenthesisLeft),
            None,
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_identifier() {
        let input = r#"#[=]"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_identifier(
            Location::test(1, 3),
            Lexeme::Symbol(Symbol::Equals),
            None,
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_bracket_square_right() {
        let input = r#"#[test)"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_one_of(
            Location::test(1, 7),
            vec!["]"],
            Lexeme::Symbol(Symbol::ParenthesisRight),
            None,
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
