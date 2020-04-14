//!
//! The binding pattern list parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::stream::TokenStream;
use crate::lexical::token::lexeme::keyword::Keyword;
use crate::lexical::token::lexeme::symbol::Symbol;
use crate::lexical::token::lexeme::Lexeme;
use crate::lexical::token::Token;
use crate::syntax::parser::pattern_binding::Parser as BindingPatternParser;
use crate::syntax::tree::pattern_binding::Pattern as BindingPattern;

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
    ///
    /// Parses a binding pattern list.
    ///
    /// 'mut a: u8, b: field, c: (bool, bool)'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(Vec<BindingPattern>, Option<Token>), Error> {
        loop {
            match self.state {
                State::BindingPattern => {
                    match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
                        token
                        @
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Mut),
                            ..
                        }
                        | token
                        @
                        Token {
                            lexeme: Lexeme::Identifier(_),
                            ..
                        }
                        | token
                        @
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Underscore),
                            ..
                        }
                        | token
                        @
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::SelfLowercase),
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
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
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
    use crate::lexical::stream::TokenStream;
    use crate::lexical::token::lexeme::Lexeme;
    use crate::lexical::token::location::Location;
    use crate::lexical::token::Token;
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::pattern_binding::variant::Variant as BindingPatternVariant;
    use crate::syntax::tree::pattern_binding::Pattern as BindingPattern;
    use crate::syntax::tree::r#type::variant::Variant as TypeVariant;
    use crate::syntax::tree::r#type::Type;

    #[test]
    fn ok_empty() {
        let input = r#""#;

        let expected = Ok((
            Vec::<BindingPattern>::new(),
            Some(Token::new(Lexeme::Eof, Location::new(1, 1))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single() {
        let input = r#"a: u232"#;

        let expected = Ok((
            vec![BindingPattern::new(
                Location::new(1, 1),
                BindingPatternVariant::new_binding(
                    Identifier::new(Location::new(1, 1), "a".to_owned()),
                    false,
                ),
                Type::new(Location::new(1, 4), TypeVariant::integer_unsigned(232)),
            )],
            Some(Token::new(Lexeme::Eof, Location::new(1, 8))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single_with_comma() {
        let input = r#"a: u232,"#;

        let expected = Ok((
            vec![BindingPattern::new(
                Location::new(1, 1),
                BindingPatternVariant::new_binding(
                    Identifier::new(Location::new(1, 1), "a".to_owned()),
                    false,
                ),
                Type::new(Location::new(1, 4), TypeVariant::integer_unsigned(232)),
            )],
            Some(Token::new(Lexeme::Eof, Location::new(1, 9))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_multiple() {
        let input = r#"a: u232, b: u8, c: field"#;

        let expected = Ok((
            vec![
                BindingPattern::new(
                    Location::new(1, 1),
                    BindingPatternVariant::new_binding(
                        Identifier::new(Location::new(1, 1), "a".to_owned()),
                        false,
                    ),
                    Type::new(Location::new(1, 4), TypeVariant::integer_unsigned(232)),
                ),
                BindingPattern::new(
                    Location::new(1, 10),
                    BindingPatternVariant::new_binding(
                        Identifier::new(Location::new(1, 10), "b".to_owned()),
                        false,
                    ),
                    Type::new(Location::new(1, 13), TypeVariant::integer_unsigned(8)),
                ),
                BindingPattern::new(
                    Location::new(1, 17),
                    BindingPatternVariant::new_binding(
                        Identifier::new(Location::new(1, 17), "c".to_owned()),
                        false,
                    ),
                    Type::new(Location::new(1, 20), TypeVariant::field()),
                ),
            ],
            Some(Token::new(Lexeme::Eof, Location::new(1, 25))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
