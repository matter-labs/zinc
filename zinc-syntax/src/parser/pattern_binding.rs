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
use crate::parser::r#type::Parser as TypeParser;
use crate::tree::identifier::Identifier;
use crate::tree::pattern_binding::builder::Builder as BindingPatternBuilder;
use crate::tree::pattern_binding::Pattern as BindingPattern;

/// The missing type error hint.
pub static HINT_EXPECTED_TYPE: &str =
    "function argument must have a type, e.g. `fn sum(a: u8, b: u8) {}`";

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    MutOrNext,
    /// The optional `mut` has been parsed so far.
    Binding,
    /// The `{identifier}` has been parsed so far.
    Colon,
    /// The `{identifier} :` has been parsed so far.
    Type,
}

impl Default for State {
    fn default() -> Self {
        Self::MutOrNext
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
    /// 'a: u8'
    /// 'mut a: u8'
    /// '_: u8'
    /// 'self'
    /// 'mut self'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(BindingPattern, Option<Token>), ParsingError> {
        loop {
            match self.state {
                State::MutOrNext => {
                    match crate::parser::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Mut),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.builder.set_mutable();
                            self.state = State::Binding;
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
                            self.state = State::Colon;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Underscore),
                            ..
                        } => {
                            self.builder.set_wildcard();
                            self.state = State::Colon;
                        }
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::SelfLowercase),
                            location,
                        } => {
                            self.builder.set_self_location(location);
                            self.builder.set_self_alias();
                            return Ok((self.builder.finish(), None));
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(
                                SyntaxError::expected_binding_pattern(location, lexeme),
                            ));
                        }
                    }
                }
                State::Colon => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Colon),
                            ..
                        } => self.state = State::Type,
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_type(
                                location,
                                lexeme,
                                Some(HINT_EXPECTED_TYPE),
                            )));
                        }
                    }
                }
                State::Type => {
                    let (r#type, next) = TypeParser::default().parse(stream, self.next.take())?;
                    self.builder.set_type(r#type);
                    return Ok((self.builder.finish(), next));
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
    use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::tree::expression::tree::Tree as ExpressionTree;
    use crate::tree::identifier::Identifier;
    use crate::tree::pattern_binding::variant::Variant as BindingPatternVariant;
    use crate::tree::pattern_binding::Pattern as BindingPattern;
    use crate::tree::r#type::variant::Variant as TypeVariant;
    use crate::tree::r#type::Type;

    #[test]
    fn ok_binding() {
        let input = r#"value: u8"#;

        let expected = Ok((
            BindingPattern::new(
                Location::test(1, 1),
                BindingPatternVariant::new_binding(
                    Identifier::new(Location::test(1, 1), "value".to_owned()),
                    false,
                ),
                Type::new(Location::test(1, 8), TypeVariant::integer_unsigned(8)),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_binding_mutable() {
        let input = r#"mut value: u8"#;

        let expected = Ok((
            BindingPattern::new(
                Location::test(1, 1),
                BindingPatternVariant::new_binding(
                    Identifier::new(Location::test(1, 5), "value".to_owned()),
                    true,
                ),
                Type::new(Location::test(1, 12), TypeVariant::integer_unsigned(8)),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_wildcard() {
        let input = r#"_: u8"#;

        let expected = Ok((
            BindingPattern::new(
                Location::test(1, 1),
                BindingPatternVariant::new_wildcard(),
                Type::new(Location::test(1, 4), TypeVariant::integer_unsigned(8)),
            ),
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
                BindingPatternVariant::new_self_alias(Location::test(1, 1), false),
                Type::new(
                    Location::test(1, 1),
                    TypeVariant::alias(ExpressionTree::new(
                        Location::test(1, 1),
                        ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                            Identifier::new(
                                Location::test(1, 1),
                                Keyword::SelfUppercase.to_string(),
                            ),
                        )),
                    )),
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
                BindingPatternVariant::new_self_alias(Location::test(1, 5), true),
                Type::new(
                    Location::test(1, 5),
                    TypeVariant::alias(ExpressionTree::new(
                        Location::test(1, 5),
                        ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                            Identifier::new(
                                Location::test(1, 5),
                                Keyword::SelfUppercase.to_string(),
                            ),
                        )),
                    )),
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

    #[test]
    fn error_expected_type() {
        let input = r#"mut value"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_type(
            Location::test(1, 10),
            Lexeme::Eof,
            Some(super::HINT_EXPECTED_TYPE),
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
