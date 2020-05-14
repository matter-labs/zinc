//!
//! The addition/subtraction operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::stream::TokenStream;
use crate::lexical::token::lexeme::symbol::Symbol;
use crate::lexical::token::lexeme::Lexeme;
use crate::lexical::token::Token;
use crate::syntax::parser::expression::mul_div_rem::Parser as MulDivRemOperandParser;
use crate::syntax::tree::expression::tree::builder::Builder as ExpressionTreeBuilder;
use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;

#[derive(Debug, Clone, Copy)]
pub enum State {
    MulDivRemOperand,
    MulDivRemOperator,
}

impl Default for State {
    fn default() -> Self {
        State::MulDivRemOperand
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    next: Option<Token>,
    builder: ExpressionTreeBuilder,
}

impl Parser {
    ///
    /// Parses a binary addition or subtraction expression operand, which is
    /// a lower precedence binary multiplication, division or remainder operator expression.
    ///
    /// '42 * 64'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(ExpressionTree, Option<Token>), Error> {
        loop {
            match self.state {
                State::MulDivRemOperand => {
                    let (expression, next) =
                        MulDivRemOperandParser::default().parse(stream.clone(), initial.take())?;
                    self.next = next;
                    self.builder.eat(expression);
                    self.state = State::MulDivRemOperator;
                }
                State::MulDivRemOperator => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Asterisk),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::Multiplication, location);
                            self.state = State::MulDivRemOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Slash),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::Division, location);
                            self.state = State::MulDivRemOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Percent),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::Remainder, location);
                            self.state = State::MulDivRemOperand;
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
    use super::Parser;
    use crate::lexical::stream::TokenStream;
    use crate::lexical::token::lexeme::literal::integer::Integer as LexicalIntegerLiteral;
    use crate::lexical::token::lexeme::Lexeme;
    use crate::lexical::token::location::Location;
    use crate::lexical::token::Token;
    use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
    use crate::syntax::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
    use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;

    #[test]
    fn ok_multiplication() {
        let input = r#"42 * 228"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::new(1, 4),
                ExpressionTreeNode::operator(ExpressionOperator::Multiplication),
                Some(ExpressionTree::new(
                    Location::new(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::new(1, 1),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                )),
                Some(ExpressionTree::new(
                    Location::new(1, 6),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::new(1, 6),
                            LexicalIntegerLiteral::new_decimal("228".to_owned()),
                        ),
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 9))),
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_division() {
        let input = r#"42 / 228"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::new(1, 4),
                ExpressionTreeNode::operator(ExpressionOperator::Division),
                Some(ExpressionTree::new(
                    Location::new(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::new(1, 1),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                )),
                Some(ExpressionTree::new(
                    Location::new(1, 6),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::new(1, 6),
                            LexicalIntegerLiteral::new_decimal("228".to_owned()),
                        ),
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 9))),
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_remainder() {
        let input = r#"42 % 228"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::new(1, 4),
                ExpressionTreeNode::operator(ExpressionOperator::Remainder),
                Some(ExpressionTree::new(
                    Location::new(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::new(1, 1),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                )),
                Some(ExpressionTree::new(
                    Location::new(1, 6),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::new(1, 6),
                            LexicalIntegerLiteral::new_decimal("228".to_owned()),
                        ),
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 9))),
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
