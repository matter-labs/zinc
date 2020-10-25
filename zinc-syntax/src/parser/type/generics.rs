//!
//! The generic type arguments parser.
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
use crate::tree::r#type::Type;

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    Lesser,
    /// The `<` has been parsed so far.
    TypeOrGreater,
    /// The `< {type}` has been parsed so far.
    CommaOrGreater,
}

impl Default for State {
    fn default() -> Self {
        Self::Lesser
    }
}

///
/// The generic type arguments parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The token returned from a subparser.
    next: Option<Token>,
    /// The parsed types.
    types: Vec<Type>,
}

impl Parser {
    ///
    /// Parses a generic type arguments list.
    ///
    /// '<u8, field, bool>'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(Vec<Type>, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::Lesser => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Lesser),
                            ..
                        } => {
                            self.state = State::TypeOrGreater;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["<"],
                                lexeme,
                                None,
                            )))
                        }
                    }
                }
                State::TypeOrGreater => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Greater),
                            ..
                        } => {
                            return Ok((self.types, self.next.take()));
                        }
                        token => {
                            let (r#type, next) =
                                TypeParser::default().parse(stream.clone(), Some(token))?;
                            self.types.push(r#type);
                            self.next = next;
                            self.state = State::CommaOrGreater;
                        }
                    }
                }
                State::CommaOrGreater => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Comma),
                            ..
                        } => self.state = State::TypeOrGreater,
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Greater),
                            ..
                        } => return Ok((self.types, self.next.take())),
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleGreater),
                            location,
                        } => {
                            // cuts off a `>` from `>>`, giving back `>`
                            return Ok((
                                self.types,
                                Some(Token {
                                    lexeme: Lexeme::Symbol(Symbol::Greater),
                                    location: location.shifted_right(1),
                                }),
                            ));
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleGreaterEquals),
                            location,
                        } => {
                            // cuts off a `>` from `>>=`, giving back `>=`
                            return Ok((
                                self.types,
                                Some(Token {
                                    lexeme: Lexeme::Symbol(Symbol::GreaterEquals),
                                    location: location.shifted_right(1),
                                }),
                            ));
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec![",", ">"],
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
    use crate::tree::identifier::Identifier;
    use crate::tree::literal::integer::Literal as IntegerLiteral;
    use crate::tree::r#type::variant::Variant as TypeVariant;
    use crate::tree::r#type::Type;

    #[test]
    fn ok_empty() {
        let input = r#"<>"#;

        let expected = Ok((vec![], None));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single() {
        let input = r#"<field>"#;

        let expected = Ok((
            vec![Type::new(Location::test(1, 2), TypeVariant::field())],
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single_with_comma() {
        let input = r#"<field,>"#;

        let expected = Ok((
            vec![Type::new(Location::test(1, 2), TypeVariant::field())],
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_multiple() {
        let input = r#"<field, (), [u8; 4]>"#;

        let expected = Ok((
            vec![
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
            ],
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_nested() {
        let input = r#"<bool, Map<u8, u248>>"#;

        let expected = Ok((
            vec![
                Type::new(Location::test(1, 2), TypeVariant::boolean()),
                Type::new(
                    Location::test(1, 8),
                    TypeVariant::alias(
                        ExpressionTree::new(
                            Location::test(1, 8),
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::test(1, 8), "Map".to_owned()),
                            )),
                        ),
                        Some(vec![
                            Type::new(
                                Location::test(1, 12),
                                TypeVariant::integer_unsigned(zinc_const::bitlength::BYTE),
                            ),
                            Type::new(
                                Location::test(1, 16),
                                TypeVariant::integer_unsigned(zinc_const::bitlength::INTEGER_MAX),
                            ),
                        ]),
                    ),
                ),
            ],
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_comma_or_greater() {
        let input = r#"<field;>"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_one_of(
            Location::test(1, 7),
            vec![",", ">"],
            Lexeme::Symbol(Symbol::Semicolon),
            None,
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
