//!
//! The tuple type parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Lexeme;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::Error as SyntaxError;
use crate::error::ParsingError;
use crate::parser::r#type::Parser as TypeParser;
use crate::tree::r#type::builder::Builder as TypeBuilder;
use crate::tree::r#type::Type;

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    ParenthesisLeft,
    /// The `(` has been parsed so far.
    TypeOrParenthesisRight,
    /// The `( {type}` has been parsed so far.
    CommaOrParenthesisRight,
}

impl Default for State {
    fn default() -> Self {
        Self::ParenthesisLeft
    }
}

///
/// The tuple type parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The token returned from a subparser.
    next: Option<Token>,
    /// The builder of the parsed type.
    builder: TypeBuilder,
}

impl Parser {
    ///
    /// Parses a tuple type literal.
    ///
    /// '(u8, field, bool)'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(Type, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::ParenthesisLeft => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::TypeOrParenthesisRight;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["("],
                                lexeme,
                                None,
                            )))
                        }
                    }
                }
                State::TypeOrParenthesisRight => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        } => {
                            self.builder.set_unit_if_empty();
                            return Ok((self.builder.finish(), self.next.take()));
                        }
                        token => {
                            let (element_type, next) =
                                TypeParser::default().parse(stream.clone(), Some(token))?;
                            self.next = next;
                            self.builder.push_tuple_element_type(element_type);
                            self.state = State::CommaOrParenthesisRight;
                        }
                    }
                }
                State::CommaOrParenthesisRight => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Comma),
                            ..
                        } => self.state = State::TypeOrParenthesisRight,
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        } => return Ok((self.builder.finish(), self.next.take())),
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
    use zinc_lexical::IntegerLiteral as LexicalIntegerLiteral;
    use zinc_lexical::Lexeme;
    use zinc_lexical::Location;
    use zinc_lexical::Symbol;
    use zinc_lexical::TokenStream;

    use super::Parser;
    use crate::error::Error as SyntaxError;
    use crate::error::ParsingError;
    use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::tree::expression::tree::Tree as ExpressionTree;
    use crate::tree::literal::integer::Literal as IntegerLiteral;
    use crate::tree::r#type::variant::Variant as TypeVariant;
    use crate::tree::r#type::Type;

    #[test]
    fn ok_unit() {
        let input = r#"()"#;

        let expected = Ok((Type::new(Location::test(1, 1), TypeVariant::unit()), None));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single() {
        let input = r#"(field)"#;

        let expected = Ok((
            Type::new(
                Location::test(1, 1),
                TypeVariant::tuple(vec![Type::new(Location::test(1, 2), TypeVariant::field())]),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single_with_comma() {
        let input = r#"(field,)"#;

        let expected = Ok((
            Type::new(
                Location::test(1, 1),
                TypeVariant::tuple(vec![Type::new(Location::test(1, 2), TypeVariant::field())]),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_multiple() {
        let input = r#"(field, (), [u8; 4])"#;

        let expected = Ok((
            Type::new(
                Location::test(1, 1),
                TypeVariant::tuple(vec![
                    Type::new(Location::test(1, 2), TypeVariant::field()),
                    Type::new(Location::test(1, 9), TypeVariant::unit()),
                    Type::new(
                        Location::test(1, 13),
                        TypeVariant::array(
                            Type::new(
                                Location::test(1, 14),
                                TypeVariant::integer_unsigned(zinc_const::bitlength::BYTE),
                            ),
                            ExpressionTree::new(
                                Location::test(1, 18),
                                ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                    IntegerLiteral::new(
                                        Location::test(1, 18),
                                        LexicalIntegerLiteral::new_decimal("4".to_owned()),
                                    ),
                                )),
                            ),
                        ),
                    ),
                ]),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_nested() {
        let input = r#"((field, field),)"#;

        let expected = Ok((
            Type::new(
                Location::test(1, 1),
                TypeVariant::tuple(vec![Type::new(
                    Location::test(1, 2),
                    TypeVariant::tuple(vec![
                        Type::new(Location::test(1, 3), TypeVariant::field()),
                        Type::new(Location::test(1, 10), TypeVariant::field()),
                    ]),
                )]),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_comma_or_parenthesis_right() {
        let input = r#"(field;)"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_one_of(
            Location::test(1, 7),
            vec![",", ")"],
            Lexeme::Symbol(Symbol::Semicolon),
            None,
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
