//!
//! The `fn` statement parser.
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
use crate::parser::binding_list::Parser as BindingListParser;
use crate::parser::expression::terminal::block::Parser as BlockExpressionParser;
use crate::parser::r#type::Parser as TypeParser;
use crate::tree::identifier::Identifier;
use crate::tree::statement::r#fn::builder::Builder as FnStatementBuilder;

/// The missing identifier error hint.
pub static HINT_EXPECTED_IDENTIFIER: &str =
    "function must have an identifier, e.g. `fn sum(...) { ... }`";
/// The missing argument list error hint.
pub static HINT_EXPECTED_ARGUMENT_LIST: &str =
    "function must have the argument list, e.g. `fn sum(a: u8, b: u8) { ... }`";

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    KeywordFn,
    /// The `fn` has been parsed so far.
    Identifier,
    /// The `fn {identifier}` has been parsed so far.
    ParenthesisLeft,
    /// The `fn {identifier} (` has been parsed so far.
    ArgumentBindingList,
    /// The `fn {identifier} ( {arguments}` has been parsed so far.
    ParenthesisRight,
    /// The `fn {identifier} ( {arguments} )` has been parsed so far.
    ArrowOrBody,
    /// The `fn {identifier} ( {arguments} ) ->` has been parsed so far.
    ReturnType,
    /// The `fn {identifier} ( {arguments} )` with optional `-> {type}` has been parsed so far.
    Body,
}

impl Default for State {
    fn default() -> Self {
        Self::KeywordFn
    }
}

///
/// The `fn` statement parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The builder of the parsed value.
    builder: FnStatementBuilder,
    /// The token returned from a subparser.
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses an 'fn' statement.
    ///
    /// '
    /// fn sum(a: u8, b: u8) -> u8 {
    ///     a + b
    /// }
    /// '
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(FnStatementBuilder, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::KeywordFn => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Fn),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::Identifier;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["fn"],
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
                            self.state = State::ParenthesisLeft;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_identifier(
                                location,
                                lexeme,
                                Some(HINT_EXPECTED_IDENTIFIER),
                            )));
                        }
                    }
                }
                State::ParenthesisLeft => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                            ..
                        } => self.state = State::ArgumentBindingList,
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["("],
                                lexeme,
                                Some(HINT_EXPECTED_ARGUMENT_LIST),
                            )));
                        }
                    }
                }
                State::ArgumentBindingList => {
                    let (argument_bindings, next) =
                        BindingListParser::default().parse(stream.clone(), self.next.take())?;
                    self.builder.set_argument_bindings(argument_bindings);
                    self.next = next;
                    self.state = State::ParenthesisRight;
                }
                State::ParenthesisRight => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        } => self.state = State::ArrowOrBody,
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec![")"],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::ArrowOrBody => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::MinusGreater),
                            ..
                        } => self.state = State::ReturnType,
                        token => {
                            self.next = Some(token);
                            self.state = State::Body;
                        }
                    }
                }
                State::ReturnType => {
                    let (r#type, next) =
                        TypeParser::default().parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.set_return_type(r#type);
                    self.state = State::Body;
                }
                State::Body => {
                    let (expression, next) =
                        BlockExpressionParser::default().parse(stream, self.next.take())?;

                    self.builder.set_body(expression);
                    return Ok((self.builder, next));
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
    use crate::tree::binding::Binding;
    use crate::tree::expression::block::Expression as BlockExpression;
    use crate::tree::identifier::Identifier;
    use crate::tree::pattern_binding::variant::Variant as BindingPatternVariant;
    use crate::tree::pattern_binding::Pattern as BindingPattern;
    use crate::tree::r#type::variant::Variant as TypeVariant;
    use crate::tree::r#type::Type;
    use crate::tree::statement::r#fn::Statement as FnStatement;

    #[test]
    fn ok_returns_unit() {
        let input = r#"fn f(a: field) {}"#;

        let expected = Ok((
            FnStatement::new(
                Location::test(1, 1),
                false,
                false,
                Identifier::new(Location::test(1, 4), "f".to_owned()),
                vec![Binding::new(
                    Location::test(1, 6),
                    BindingPattern::new(
                        Location::test(1, 6),
                        BindingPatternVariant::new_binding(
                            Identifier::new(Location::test(1, 6), "a".to_owned()),
                            false,
                        ),
                    ),
                    Some(Type::new(Location::test(1, 9), TypeVariant::field())),
                )],
                None,
                BlockExpression::new(Location::test(1, 16), vec![], None),
                vec![],
            ),
            None,
        ));

        let result = Parser::default()
            .parse(TokenStream::test(input).wrap(), None)
            .map(|(builder, next)| (builder.finish(), next));

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_returns_type() {
        let input = r#"fn f(a: field) -> field {}"#;

        let expected = Ok((
            FnStatement::new(
                Location::test(1, 1),
                false,
                false,
                Identifier::new(Location::test(1, 4), "f".to_owned()),
                vec![Binding::new(
                    Location::test(1, 6),
                    BindingPattern::new(
                        Location::test(1, 6),
                        BindingPatternVariant::new_binding(
                            Identifier::new(Location::test(1, 6), "a".to_owned()),
                            false,
                        ),
                    ),
                    Some(Type::new(Location::test(1, 9), TypeVariant::field())),
                )],
                Some(Type::new(Location::test(1, 19), TypeVariant::field())),
                BlockExpression::new(Location::test(1, 25), vec![], None),
                vec![],
            ),
            None,
        ));

        let result = Parser::default()
            .parse(TokenStream::test(input).wrap(), None)
            .map(|(builder, next)| (builder.finish(), next));

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_identifier() {
        let input = r#"fn (a: u8) -> field {}"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_identifier(
            Location::test(1, 4),
            Lexeme::Symbol(Symbol::ParenthesisLeft),
            Some(super::HINT_EXPECTED_IDENTIFIER),
        )));

        let result = Parser::default()
            .parse(TokenStream::test(input).wrap(), None)
            .map(|(builder, next)| (builder.finish(), next));

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_parenthesis_left() {
        let input = r#"fn sort -> field {}"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_one_of(
            Location::test(1, 9),
            vec!["("],
            Lexeme::Symbol(Symbol::MinusGreater),
            Some(super::HINT_EXPECTED_ARGUMENT_LIST),
        )));

        let result = Parser::default()
            .parse(TokenStream::test(input).wrap(), None)
            .map(|(builder, next)| (builder.finish(), next));

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_comma_or_parenthesis_right() {
        let input = r#"fn sort(array: [u8; 100]] -> field {}"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_one_of(
            Location::test(1, 25),
            vec![")"],
            Lexeme::Symbol(Symbol::BracketSquareRight),
            None,
        )));

        let result = Parser::default()
            .parse(TokenStream::test(input).wrap(), None)
            .map(|(builder, next)| (builder.finish(), next));

        assert_eq!(result, expected);
    }
}
