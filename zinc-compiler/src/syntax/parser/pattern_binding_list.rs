//!
//! The binding pattern list parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::BindingPattern;
use crate::syntax::BindingPatternParser;

#[derive(Debug, Clone, Copy)]
pub enum State {
    BindingPattern,
    CommaOrEnd,
}

impl Default for State {
    fn default() -> Self {
        State::BindingPattern
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    patterns: Vec<BindingPattern>,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(Vec<BindingPattern>, Option<Token>), Error> {
        loop {
            match self.state {
                State::BindingPattern => {
                    match match initial.take() {
                        Some(token) => token,
                        None => stream.borrow_mut().next()?,
                    } {
                        token
                        @
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Mut),
                            ..
                        } => {
                            let (pattern, next) = BindingPatternParser::default()
                                .parse(stream.clone(), Some(token))?;
                            self.next = next;
                            self.patterns.push(pattern);
                        }
                        token
                        @
                        Token {
                            lexeme: Lexeme::Identifier(_),
                            ..
                        } => {
                            let (pattern, next) = BindingPatternParser::default()
                                .parse(stream.clone(), Some(token))?;
                            self.next = next;
                            self.patterns.push(pattern);
                        }
                        token
                        @
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Underscore),
                            ..
                        } => {
                            let (pattern, next) = BindingPatternParser::default()
                                .parse(stream.clone(), Some(token))?;
                            self.next = next;
                            self.patterns.push(pattern);
                        }
                        token => self.next = Some(token),
                    }
                    self.state = State::CommaOrEnd;
                }
                State::CommaOrEnd => {
                    match match self.next.take() {
                        Some(token) => token,
                        None => stream.borrow_mut().next()?,
                    } {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Comma),
                            ..
                        } => self.state = State::BindingPattern,
                        token => return Ok((self.patterns, Some(token))),
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
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::BindingPattern;
    use crate::syntax::BindingPatternVariant;
    use crate::syntax::Identifier;
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;

    #[test]
    fn ok_single() {
        let input = r#"a: u232"#;

        let expected = Ok((
            vec![BindingPattern::new(
                Location::new(1, 1),
                BindingPatternVariant::Binding(Identifier::new(
                    Location::new(1, 1),
                    "a".to_owned(),
                )),
                Type::new(Location::new(1, 4), TypeVariant::new_integer_unsigned(232)),
            )],
            Some(Token::new(Lexeme::Eof, Location::new(1, 8))),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_single_with_comma() {
        let input = r#"a: u232,"#;

        let expected = Ok((
            vec![BindingPattern::new(
                Location::new(1, 1),
                BindingPatternVariant::Binding(Identifier::new(
                    Location::new(1, 1),
                    "a".to_owned(),
                )),
                Type::new(Location::new(1, 4), TypeVariant::new_integer_unsigned(232)),
            )],
            Some(Token::new(Lexeme::Eof, Location::new(1, 9))),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_multiple() {
        let input = r#"a: u232, b: u8, c: field"#;

        let expected = Ok((
            vec![
                BindingPattern::new(
                    Location::new(1, 1),
                    BindingPatternVariant::Binding(Identifier::new(
                        Location::new(1, 1),
                        "a".to_owned(),
                    )),
                    Type::new(Location::new(1, 4), TypeVariant::new_integer_unsigned(232)),
                ),
                BindingPattern::new(
                    Location::new(1, 10),
                    BindingPatternVariant::Binding(Identifier::new(
                        Location::new(1, 10),
                        "b".to_owned(),
                    )),
                    Type::new(Location::new(1, 13), TypeVariant::new_integer_unsigned(8)),
                ),
                BindingPattern::new(
                    Location::new(1, 17),
                    BindingPatternVariant::Binding(Identifier::new(
                        Location::new(1, 17),
                        "c".to_owned(),
                    )),
                    Type::new(Location::new(1, 20), TypeVariant::new_field()),
                ),
            ],
            Some(Token::new(Lexeme::Eof, Location::new(1, 25))),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_empty() {
        let input = r#""#;

        let expected = Ok((
            Vec::<BindingPattern>::new(),
            Some(Token::new(Lexeme::Eof, Location::new(1, 1))),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }
}
