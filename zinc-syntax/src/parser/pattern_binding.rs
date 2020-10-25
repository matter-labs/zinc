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
use crate::tree::identifier::Identifier;
use crate::tree::pattern_binding::builder::Builder as BindingPatternBuilder;
use crate::tree::pattern_binding::Pattern as BindingPattern;

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    Initial,
    /// The optional `mut` has been parsed so far.
    Binding,
    /// The list is being parsed here.
    BindingOrParenthesisRight,
    /// The `( {binding}` has been parsed so far.
    CommaOrParenthesisRight,
}

impl Default for State {
    fn default() -> Self {
        Self::Initial
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
    builder: BindingPatternBuilder,
    /// The token returned from a subparser.
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses a binding pattern.
    ///
    /// 'a'
    /// '(a, b, c)'
    /// 'mut a'
    /// '(mut a, b, mut c)'
    /// '_'
    /// 'self'
    /// 'mut self'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(BindingPattern, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::Initial => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Mut),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.builder.set_mutable();
                            self.state = State::Binding;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                            location,
                        } => {
                            self.builder.set_location(location);

                            self.state = State::BindingOrParenthesisRight;
                        }
                        token => {
                            self.builder.set_location(token.location);
                            self.next = Some(token);
                            self.state = State::Binding;
                        }
                    }
                }
                State::Binding => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            self.builder
                                .set_identifier(Identifier::new(location, identifier.inner));
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Underscore),
                            ..
                        } => {
                            self.builder.set_wildcard();
                        }
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::SelfLowercase),
                            location,
                        } => {
                            self.builder.set_identifier(Identifier::new(
                                location,
                                Keyword::SelfLowercase.to_string(),
                            ));
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(
                                SyntaxError::expected_binding_pattern(location, lexeme),
                            ));
                        }
                    }

                    return Ok((self.builder.finish(), None));
                }
                State::BindingOrParenthesisRight => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        } => {
                            return Ok((self.builder.finish(), None));
                        }
                        token => {
                            let (binding, next) =
                                Self::default().parse(stream.clone(), Some(token))?;
                            self.builder.push_binding(binding);
                            self.next = next;

                            self.state = State::CommaOrParenthesisRight;
                        }
                    }
                }
                State::CommaOrParenthesisRight => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Comma),
                            ..
                        } => {
                            self.state = State::BindingOrParenthesisRight;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        } => {
                            return Ok((self.builder.finish(), None));
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec![",", ")"],
                                lexeme,
                                None,
                            )))
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use zinc_lexical::Keyword;
    use zinc_lexical::Lexeme;
    use zinc_lexical::Location;
    use zinc_lexical::TokenStream;

    use super::Parser;
    use crate::error::Error as SyntaxError;
    use crate::error::ParsingError;
    use crate::tree::identifier::Identifier;
    use crate::tree::pattern_binding::variant::Variant as BindingPatternVariant;
    use crate::tree::pattern_binding::Pattern as BindingPattern;

    #[test]
    fn ok() {
        let input = r#"value"#;

        let expected = Ok((
            BindingPattern::new(
                Location::test(1, 1),
                BindingPatternVariant::new_binding(
                    Identifier::new(Location::test(1, 1), "value".to_owned()),
                    false,
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_mutable() {
        let input = r#"mut value"#;

        let expected = Ok((
            BindingPattern::new(
                Location::test(1, 1),
                BindingPatternVariant::new_binding(
                    Identifier::new(Location::test(1, 5), "value".to_owned()),
                    true,
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_wildcard() {
        let input = r#"_"#;

        let expected = Ok((
            BindingPattern::new(Location::test(1, 1), BindingPatternVariant::new_wildcard()),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_self_alias() {
        let input = r#"self"#;

        let expected = Ok((
            BindingPattern::new(
                Location::test(1, 1),
                BindingPatternVariant::new_binding(
                    Identifier::new(Location::test(1, 1), Keyword::SelfLowercase.to_string()),
                    false,
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_self_alias_mutable() {
        let input = r#"mut self"#;

        let expected = Ok((
            BindingPattern::new(
                Location::test(1, 1),
                BindingPatternVariant::new_binding(
                    Identifier::new(Location::test(1, 5), Keyword::SelfLowercase.to_string()),
                    true,
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_binding_pattern() {
        let input = r#"mut bool: bool"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_binding_pattern(
            Location::test(1, 5),
            Lexeme::Keyword(Keyword::Bool),
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
