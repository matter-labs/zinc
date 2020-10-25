//!
//! The binding pattern list parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Keyword;
use zinc_lexical::Lexeme;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::ParsingError;
use crate::parser::binding::Parser as BindingParser;
use crate::tree::binding::Binding;

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    Binding,
    /// The `{binding}` has been parsed so far. A comma prepends the next binding pattern.
    CommaOrEnd,
}

impl Default for State {
    fn default() -> Self {
        Self::Binding
    }
}

///
/// The binding pattern list parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The parsed bindings.
    bindings: Vec<Binding>,
    /// The token returned from a subparser.
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
        initial: Option<Token>,
    ) -> Result<(Vec<Binding>, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::Binding => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        token
                        @
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Mut),
                            ..
                        }
                        | token
                        @
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
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
                            let (binding, next) =
                                BindingParser::default().parse(stream.clone(), Some(token))?;
                            self.bindings.push(binding);
                            self.next = next;

                            self.state = State::CommaOrEnd;
                        }
                        token => return Ok((self.bindings, Some(token))),
                    }
                }
                State::CommaOrEnd => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Comma),
                            ..
                        } => {
                            self.state = State::Binding;
                        }
                        token => return Ok((self.bindings, Some(token))),
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
    use zinc_lexical::Token;
    use zinc_lexical::TokenStream;

    use super::Parser;
    use crate::tree::binding::Binding;
    use crate::tree::identifier::Identifier;
    use crate::tree::pattern_binding::variant::Variant as BindingPatternVariant;
    use crate::tree::pattern_binding::Pattern as BindingPattern;
    use crate::tree::r#type::variant::Variant as TypeVariant;
    use crate::tree::r#type::Type;

    #[test]
    fn ok_empty() {
        let input = r#""#;

        let expected = Ok((
            Vec::<Binding>::new(),
            Some(Token::new(Lexeme::Eof, Location::test(1, 1))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single() {
        let input = r#"a: u232"#;

        let expected = Ok((
            vec![Binding::new(
                Location::test(1, 1),
                BindingPattern::new(
                    Location::test(1, 1),
                    BindingPatternVariant::new_binding(
                        Identifier::new(Location::test(1, 1), "a".to_owned()),
                        false,
                    ),
                ),
                Some(Type::new(
                    Location::test(1, 4),
                    TypeVariant::integer_unsigned(232),
                )),
            )],
            Some(Token::new(Lexeme::Eof, Location::test(1, 8))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single_with_comma() {
        let input = r#"a: u232,"#;

        let expected = Ok((
            vec![Binding::new(
                Location::test(1, 1),
                BindingPattern::new(
                    Location::test(1, 1),
                    BindingPatternVariant::new_binding(
                        Identifier::new(Location::test(1, 1), "a".to_owned()),
                        false,
                    ),
                ),
                Some(Type::new(
                    Location::test(1, 4),
                    TypeVariant::integer_unsigned(232),
                )),
            )],
            Some(Token::new(Lexeme::Eof, Location::test(1, 9))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_multiple() {
        let input = r#"a: u232, b: u8, c: field"#;

        let expected = Ok((
            vec![
                Binding::new(
                    Location::test(1, 1),
                    BindingPattern::new(
                        Location::test(1, 1),
                        BindingPatternVariant::new_binding(
                            Identifier::new(Location::test(1, 1), "a".to_owned()),
                            false,
                        ),
                    ),
                    Some(Type::new(
                        Location::test(1, 4),
                        TypeVariant::integer_unsigned(232),
                    )),
                ),
                Binding::new(
                    Location::test(1, 10),
                    BindingPattern::new(
                        Location::test(1, 10),
                        BindingPatternVariant::new_binding(
                            Identifier::new(Location::test(1, 10), "b".to_owned()),
                            false,
                        ),
                    ),
                    Some(Type::new(
                        Location::test(1, 13),
                        TypeVariant::integer_unsigned(zinc_const::bitlength::BYTE),
                    )),
                ),
                Binding::new(
                    Location::test(1, 17),
                    BindingPattern::new(
                        Location::test(1, 17),
                        BindingPatternVariant::new_binding(
                            Identifier::new(Location::test(1, 17), "c".to_owned()),
                            false,
                        ),
                    ),
                    Some(Type::new(Location::test(1, 20), TypeVariant::field())),
                ),
            ],
            Some(Token::new(Lexeme::Eof, Location::test(1, 25))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
