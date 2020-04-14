//!
//! The tuple type parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::stream::TokenStream;
use crate::lexical::token::lexeme::symbol::Symbol;
use crate::lexical::token::lexeme::Lexeme;
use crate::lexical::token::Token;
use crate::syntax::error::Error as SyntaxError;
use crate::syntax::parser::r#type::Parser as TypeParser;
use crate::syntax::tree::r#type::builder::Builder as TypeBuilder;
use crate::syntax::tree::r#type::Type;

#[derive(Debug, Clone, Copy)]
pub enum State {
    ParenthesisLeft,
    TypeOrParenthesisRight,
    CommaOrParenthesisRight,
}

impl Default for State {
    fn default() -> Self {
        State::ParenthesisLeft
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    next: Option<Token>,
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
        mut initial: Option<Token>,
    ) -> Result<(Type, Option<Token>), Error> {
        loop {
            match self.state {
                State::ParenthesisLeft => {
                    match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::TypeOrParenthesisRight;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["("],
                                lexeme,
                                None,
                            )))
                        }
                    }
                }
                State::TypeOrParenthesisRight => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
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
                            self.builder.push_tuple_element_type(element_type.variant);
                            self.state = State::CommaOrParenthesisRight;
                        }
                    }
                }
                State::CommaOrParenthesisRight => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Comma),
                            ..
                        } => self.state = State::TypeOrParenthesisRight,
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        } => return Ok((self.builder.finish(), self.next.take())),
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of(
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
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::error::Error;
    use crate::lexical::stream::TokenStream;
    use crate::lexical::token::lexeme::literal::integer::Integer as LexicalIntegerLiteral;
    use crate::lexical::token::lexeme::symbol::Symbol;
    use crate::lexical::token::lexeme::Lexeme;
    use crate::lexical::token::location::Location;
    use crate::syntax::error::Error as SyntaxError;
    use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
    use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
    use crate::syntax::tree::r#type::variant::Variant as TypeVariant;
    use crate::syntax::tree::r#type::Type;

    #[test]
    fn ok_unit() {
        let input = r#"()"#;

        let expected = Ok((Type::new(Location::new(1, 1), TypeVariant::unit()), None));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single() {
        let input = r#"(field)"#;

        let expected = Ok((
            Type::new(
                Location::new(1, 1),
                TypeVariant::tuple(vec![TypeVariant::Field]),
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single_with_comma() {
        let input = r#"(field,)"#;

        let expected = Ok((
            Type::new(
                Location::new(1, 1),
                TypeVariant::tuple(vec![TypeVariant::Field]),
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_multiple() {
        let input = r#"(field, (), [u8; 4])"#;

        let expected = Ok((
            Type::new(
                Location::new(1, 1),
                TypeVariant::tuple(vec![
                    TypeVariant::Field,
                    TypeVariant::Unit,
                    TypeVariant::array(
                        TypeVariant::integer_unsigned(8),
                        ExpressionTree::new(
                            Location::new(1, 18),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(1, 18),
                                    LexicalIntegerLiteral::new_decimal("4".to_owned()),
                                ),
                            )),
                        ),
                    ),
                ]),
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_nested() {
        let input = r#"((field, field),)"#;

        let expected = Ok((
            Type::new(
                Location::new(1, 1),
                TypeVariant::tuple(vec![TypeVariant::tuple(vec![
                    TypeVariant::Field,
                    TypeVariant::Field,
                ])]),
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_comma_or_parenthesis_right() {
        let input = r#"(field;)"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_one_of(
            Location::new(1, 7),
            vec![",", ")"],
            Lexeme::Symbol(Symbol::Semicolon),
            None,
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
