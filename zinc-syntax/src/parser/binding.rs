//!
//! The binding pattern parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Keyword;
use zinc_lexical::Lexeme;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::Error as SyntaxError;
use crate::error::ParsingError;
use crate::parser::pattern_binding::Parser as BindingPatternParser;
use crate::parser::r#type::Parser as TypeParser;
use crate::tree::binding::builder::Builder as BindingBuilder;
use crate::tree::binding::Binding;

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    Binding,
    /// The `{binding}` has been parsed so far.
    ColorOrEnd,
    /// The `{binding} :` has been parsed so far.
    Type,
}

impl Default for State {
    fn default() -> Self {
        Self::Binding
    }
}

///
/// The binding pattern parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The builder of the parsed value.
    builder: BindingBuilder,
    /// The token returned from a subparser.
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses a binding pattern.
    ///
    /// 'a'
    /// 'mut a: u8'
    /// '_: bool'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(Binding, Option<Token>), ParsingError> {
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
                            self.builder.set_location(token.location);

                            let (pattern, next) = BindingPatternParser::default()
                                .parse(stream.clone(), Some(token))?;
                            self.builder.set_pattern(pattern);
                            self.next = next;

                            self.state = State::ColorOrEnd;
                        }
                        Token { location, lexeme } => {
                            return Err(ParsingError::Syntax(
                                SyntaxError::expected_binding_pattern(location, lexeme),
                            ))
                        }
                    }
                }
                State::ColorOrEnd => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Colon),
                            ..
                        } => {
                            self.state = State::Type;
                        }
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
                State::Type => {
                    let (r#type, next) =
                        TypeParser::default().parse(stream.clone(), self.next.take())?;
                    self.builder.set_type(r#type);

                    return Ok((self.builder.finish(), next));
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
    fn ok() {
        let input = r#"value"#;

        let expected = Ok((
            Binding::new(
                Location::test(1, 1),
                BindingPattern::new(
                    Location::test(1, 1),
                    BindingPatternVariant::new_binding(
                        Identifier::new(Location::test(1, 1), "value".to_owned()),
                        false,
                    ),
                ),
                None,
            ),
            Some(Token::new(Lexeme::Eof, Location::test(1, 6))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_with_type() {
        let input = r#"value: u8"#;

        let expected = Ok((
            Binding::new(
                Location::test(1, 1),
                BindingPattern::new(
                    Location::test(1, 1),
                    BindingPatternVariant::new_binding(
                        Identifier::new(Location::test(1, 1), "value".to_owned()),
                        false,
                    ),
                ),
                Some(Type::new(
                    Location::test(1, 8),
                    TypeVariant::integer_unsigned(zinc_const::bitlength::BYTE),
                )),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
