//!
//! The attribute parser.
//!

pub mod element;
pub mod list;

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Lexeme;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::Error as SyntaxError;
use crate::error::ParsingError;
use crate::tree::attribute::builder::Builder as AttributeBuilder;
use crate::tree::attribute::Attribute;

use self::list::Parser as ElementListParser;

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    NumberSign,
    /// The `#` has been parsed so far.
    ExclamationMarkOrNext,
    /// The `#` with an optional `!` have been parsed so far.
    BracketSquareLeft,
    /// The `#[` has been parsed so far.
    Elements,
    /// The `#[{element1}, ... {elementN}]` has been parsed so far.
    BracketSquareRight,
}

impl Default for State {
    fn default() -> Self {
        Self::NumberSign
    }
}

///
/// The attribute parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The builder of the parsed value.
    builder: AttributeBuilder,
    /// The token returned from a subparser.
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses an attribute.
    ///
    /// '#[test(default)]'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(Attribute, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::NumberSign => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Number),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::ExclamationMarkOrNext;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["#"],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::ExclamationMarkOrNext => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ExclamationMark),
                            ..
                        } => {
                            self.builder.set_inner();
                        }
                        token => {
                            self.next = Some(token);
                        }
                    }

                    self.state = State::BracketSquareLeft;
                }
                State::BracketSquareLeft => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareLeft),
                            ..
                        } => self.state = State::Elements,
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["["],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::Elements => {
                    let (elements, next) =
                        ElementListParser::default().parse(stream.clone(), self.next.take())?;
                    self.builder.set_elements(elements);
                    self.next = next;
                    self.state = State::BracketSquareRight;
                }
                State::BracketSquareRight => {
                    return match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        } => Ok((self.builder.finish(), None)),
                        Token { lexeme, location } => Err(ParsingError::Syntax(
                            SyntaxError::expected_one_of(location, vec!["]"], lexeme, None),
                        )),
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
    use zinc_lexical::Symbol;
    use zinc_lexical::TokenStream;

    use super::Parser;
    use crate::error::Error as SyntaxError;
    use crate::error::ParsingError;
    use crate::tree::attribute::element::variant::Variant as AttributeElementVariant;
    use crate::tree::attribute::element::Element as AttributeElement;
    use crate::tree::attribute::Attribute;
    use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::tree::expression::tree::node::operator::Operator as ExpressionOperator;
    use crate::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::tree::expression::tree::Tree as ExpressionTree;
    use crate::tree::identifier::Identifier;
    use crate::tree::literal::integer::Literal as IntegerLiteral;
    use crate::tree::literal::Literal;

    #[test]
    fn ok_single() {
        let input = r#"#[test]"#;

        let expected = Ok((
            Attribute::new(
                Location::test(1, 1),
                false,
                vec![AttributeElement::new(
                    Location::test(1, 3),
                    ExpressionTree::new(
                        Location::test(1, 3),
                        ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                            Identifier::new(Location::test(1, 3), "test".to_owned()),
                        )),
                    ),
                    None,
                )],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_multiple() {
        let input = r#"#[test, two, three]"#;

        let expected = Ok((
            Attribute::new(
                Location::test(1, 1),
                false,
                vec![
                    AttributeElement::new(
                        Location::test(1, 3),
                        ExpressionTree::new(
                            Location::test(1, 3),
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::test(1, 3), "test".to_owned()),
                            )),
                        ),
                        None,
                    ),
                    AttributeElement::new(
                        Location::test(1, 9),
                        ExpressionTree::new(
                            Location::test(1, 9),
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::test(1, 9), "two".to_owned()),
                            )),
                        ),
                        None,
                    ),
                    AttributeElement::new(
                        Location::test(1, 14),
                        ExpressionTree::new(
                            Location::test(1, 14),
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::test(1, 14), "three".to_owned()),
                            )),
                        ),
                        None,
                    ),
                ],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_multiple_with_variants() {
        let input = r#"#[test, two = 42, three(default)]"#;

        let expected = Ok((
            Attribute::new(
                Location::test(1, 1),
                false,
                vec![
                    AttributeElement::new(
                        Location::test(1, 3),
                        ExpressionTree::new(
                            Location::test(1, 3),
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::test(1, 3), "test".to_owned()),
                            )),
                        ),
                        None,
                    ),
                    AttributeElement::new(
                        Location::test(1, 9),
                        ExpressionTree::new(
                            Location::test(1, 9),
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::test(1, 9), "two".to_owned()),
                            )),
                        ),
                        Some(AttributeElementVariant::Value(Literal::Integer(
                            IntegerLiteral::new(
                                Location::test(1, 15),
                                zinc_lexical::IntegerLiteral::new_decimal("42".to_owned()),
                            ),
                        ))),
                    ),
                    AttributeElement::new(
                        Location::test(1, 19),
                        ExpressionTree::new(
                            Location::test(1, 19),
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::test(1, 19), "three".to_owned()),
                            )),
                        ),
                        Some(AttributeElementVariant::Nested(vec![
                            AttributeElement::new(
                                Location::test(1, 25),
                                ExpressionTree::new(
                                    Location::test(1, 25),
                                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                        Identifier::new(
                                            Location::test(1, 25),
                                            "default".to_owned(),
                                        ),
                                    )),
                                ),
                                None,
                            ),
                        ])),
                    ),
                ],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_path_zksync_msg() {
        let input = r#"#[zksync::msg]"#;

        let expected = Ok((
            Attribute::new(
                Location::test(1, 1),
                false,
                vec![AttributeElement::new(
                    Location::test(1, 3),
                    ExpressionTree::new_with_leaves(
                        Location::test(1, 9),
                        ExpressionTreeNode::operator(ExpressionOperator::Path),
                        Some(ExpressionTree::new(
                            Location::test(1, 3),
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::test(1, 3), "zksync".to_owned()),
                            )),
                        )),
                        Some(ExpressionTree::new(
                            Location::test(1, 11),
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(
                                    Location::test(1, 11),
                                    zinc_const::contract::TRANSACTION_VARIABLE_NAME.to_owned(),
                                ),
                            )),
                        )),
                    ),
                    None,
                )],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_inner() {
        let input = r#"#![test]"#;

        let expected = Ok((
            Attribute::new(
                Location::test(1, 1),
                true,
                vec![AttributeElement::new(
                    Location::test(1, 4),
                    ExpressionTree::new(
                        Location::test(1, 4),
                        ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                            Identifier::new(Location::test(1, 4), "test".to_owned()),
                        )),
                    ),
                    None,
                )],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_bracket_square_left() {
        let input = r#"#(test]"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_one_of(
            Location::test(1, 2),
            vec!["["],
            Lexeme::Symbol(Symbol::ParenthesisLeft),
            None,
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_element() {
        let input = r#"#[=]"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_one_of(
            Location::test(1, 3),
            vec!["]"],
            Lexeme::Symbol(Symbol::Equals),
            None,
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_bracket_square_right() {
        let input = r#"#[test)"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_one_of(
            Location::test(1, 7),
            vec!["]"],
            Lexeme::Symbol(Symbol::ParenthesisRight),
            None,
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
