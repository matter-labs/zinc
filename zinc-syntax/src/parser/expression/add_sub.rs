//!
//! The addition/subtraction operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Lexeme;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::ParsingError;
use crate::parser::expression::mul_div_rem::Parser as MulDivRemOperandParser;
use crate::tree::expression::tree::builder::Builder as ExpressionTreeBuilder;
use crate::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::tree::expression::tree::Tree as ExpressionTree;

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    MulDivRemOperand,
    /// The operand has been parsed and an operator is expected.
    MulDivRemOperator,
}

impl Default for State {
    fn default() -> Self {
        Self::MulDivRemOperand
    }
}

///
/// The addition/subtraction operand parser.
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
    /// Parses a binary addition or subtraction expression operand, which is
    /// a lower precedence binary multiplication, division or remainder operator expression.
    ///
    /// '42 * 64'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(ExpressionTree, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::MulDivRemOperand => {
                    let (expression, next) = MulDivRemOperandParser::default()
                        .parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.eat(expression);
                    self.state = State::MulDivRemOperator;
                }
                State::MulDivRemOperator => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
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
    fn ok_multiplication() {
        let input = r#"42 * 228"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::test(1, 4),
                ExpressionTreeNode::operator(ExpressionOperator::Multiplication),
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
                    Location::test(1, 6),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::test(1, 6),
                            LexicalIntegerLiteral::new_decimal("228".to_owned()),
                        ),
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::test(1, 9))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_division() {
        let input = r#"42 / 228"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::test(1, 4),
                ExpressionTreeNode::operator(ExpressionOperator::Division),
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
                    Location::test(1, 6),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::test(1, 6),
                            LexicalIntegerLiteral::new_decimal("228".to_owned()),
                        ),
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::test(1, 9))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_remainder() {
        let input = r#"42 % 228"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::test(1, 4),
                ExpressionTreeNode::operator(ExpressionOperator::Remainder),
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
                    Location::test(1, 6),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::test(1, 6),
                            LexicalIntegerLiteral::new_decimal("228".to_owned()),
                        ),
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::test(1, 9))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
