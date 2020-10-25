//!
//! The bitwise AND operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Lexeme;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::ParsingError;
use crate::parser::expression::bitwise_shift::Parser as BitwiseShiftOperandParser;
use crate::tree::expression::tree::builder::Builder as ExpressionTreeBuilder;
use crate::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::tree::expression::tree::Tree as ExpressionTree;

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    BitwiseShiftOperand,
    /// The operand has been parsed and an operator is expected.
    BitwiseShiftOperator,
}

impl Default for State {
    fn default() -> Self {
        Self::BitwiseShiftOperand
    }
}

///
/// The bitwise AND operand parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The token returned from a subparser.
    next: Option<Token>,
    /// The builder of the parsed value.
    builder: ExpressionTreeBuilder,
}

impl Parser {
    ///
    /// Parses a bitwise AND expression operand, which is
    /// a lower precedence bitwise shift operator expression.
    ///
    /// '0b00001111 << 4'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(ExpressionTree, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::BitwiseShiftOperand => {
                    let (expression, next) = BitwiseShiftOperandParser::default()
                        .parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.eat(expression);
                    self.state = State::BitwiseShiftOperator;
                }
                State::BitwiseShiftOperator => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleLesser),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::BitwiseShiftLeft, location);
                            self.state = State::BitwiseShiftOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleGreater),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::BitwiseShiftRight, location);
                            self.state = State::BitwiseShiftOperand;
                        }
                        token => return Ok((self.builder.finish(), Some(token))),
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
    use zinc_lexical::Token;
    use zinc_lexical::TokenStream;

    use super::Parser;
    use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::tree::expression::tree::node::operator::Operator as ExpressionOperator;
    use crate::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::tree::expression::tree::Tree as ExpressionTree;
    use crate::tree::literal::integer::Literal as IntegerLiteral;

    #[test]
    fn ok_shift_left() {
        let input = r#"42 << 2"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::test(1, 4),
                ExpressionTreeNode::operator(ExpressionOperator::BitwiseShiftLeft),
                Some(ExpressionTree::new(
                    Location::test(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::test(1, 1),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                )),
                Some(ExpressionTree::new(
                    Location::test(1, 7),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::test(1, 7),
                            LexicalIntegerLiteral::new_decimal("2".to_owned()),
                        ),
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::test(1, 8))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_shift_right() {
        let input = r#"42 >> 2"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::test(1, 4),
                ExpressionTreeNode::operator(ExpressionOperator::BitwiseShiftRight),
                Some(ExpressionTree::new(
                    Location::test(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::test(1, 1),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                )),
                Some(ExpressionTree::new(
                    Location::test(1, 7),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::test(1, 7),
                            LexicalIntegerLiteral::new_decimal("2".to_owned()),
                        ),
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::test(1, 8))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
