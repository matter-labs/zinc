//!
//! The enum statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::EnumStatement;
use crate::syntax::EnumStatementBuilder;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Identifier;
use crate::syntax::VariantListParser;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordEnum,
    Identifier,
    BracketCurlyLeftOrEnd,
    VariantList,
    BracketCurlyRight,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordEnum
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: EnumStatementBuilder,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(EnumStatement, Option<Token>), Error> {
        loop {
            match self.state {
                State::KeywordEnum => {
                    match match initial.take() {
                        Some(token) => token,
                        None => stream.borrow_mut().next()?,
                    } {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Enum),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::Identifier;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["enum"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::Identifier => {
                    let next = stream.borrow_mut().next()?;
                    match next {
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            let identifier = Identifier::new(location, identifier.name);
                            self.builder.set_identifier(identifier);
                            self.state = State::BracketCurlyLeftOrEnd;
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
                State::BracketCurlyLeftOrEnd => {
                    let next = stream.borrow_mut().next()?;
                    match next {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            ..
                        } => self.state = State::VariantList,
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
                State::VariantList => {
                    let (variants, next) =
                        VariantListParser::default().parse(stream.clone(), None)?;
                    self.builder.set_variants(variants);
                    self.next = next;
                    self.state = State::BracketCurlyRight;
                }
                State::BracketCurlyRight => {
                    match self.next.take().expect("Always contains a value") {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        } => return Ok((self.builder.finish(), None)),
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["}"],
                                lexeme,
                            )));
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::lexical::IntegerLiteral;
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Symbol;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::EnumStatement;
    use crate::syntax::Identifier;
    use crate::syntax::Variant;

    #[test]
    fn ok_single() {
        let input = r#"
    enum Test {
        a = 1,
    }
"#;

        let expected = Ok((
            EnumStatement::new(
                Location::new(2, 5),
                Identifier::new(Location::new(2, 10), "Test".to_owned()),
                vec![Variant::new(
                    Location::new(3, 9),
                    Identifier::new(Location::new(3, 9), "a".to_owned()),
                    IntegerLiteral::new_decimal("1".to_owned()),
                )],
            ),
            None,
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_multiple() {
        let input = r#"
    enum Test {
        a = 1,
        b = 2,
        c = 3,
    }
"#;

        let expected = Ok((
            EnumStatement::new(
                Location::new(2, 5),
                Identifier::new(Location::new(2, 10), "Test".to_owned()),
                vec![
                    Variant::new(
                        Location::new(3, 9),
                        Identifier::new(Location::new(3, 9), "a".to_owned()),
                        IntegerLiteral::new_decimal("1".to_owned()),
                    ),
                    Variant::new(
                        Location::new(4, 9),
                        Identifier::new(Location::new(4, 9), "b".to_owned()),
                        IntegerLiteral::new_decimal("2".to_owned()),
                    ),
                    Variant::new(
                        Location::new(5, 9),
                        Identifier::new(Location::new(5, 9), "c".to_owned()),
                        IntegerLiteral::new_decimal("3".to_owned()),
                    ),
                ],
            ),
            None,
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_empty_with_brackets() {
        let input = r#"
    enum Test {}
"#;

        let expected = Ok((
            EnumStatement::new(
                Location::new(2, 5),
                Identifier::new(Location::new(2, 10), "Test".to_owned()),
                vec![],
            ),
            None,
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_empty_with_semicolon() {
        let input = r#"
    enum Test;
"#;

        let expected = Ok((
            EnumStatement::new(
                Location::new(2, 5),
                Identifier::new(Location::new(2, 10), "Test".to_owned()),
                vec![],
            ),
            Some(Token::new(
                Lexeme::Symbol(Symbol::Semicolon),
                Location::new(2, 14),
            )),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }
}
