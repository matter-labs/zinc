//!
//! The bitwise AND operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::parser::expression::bitwise_shift::Parser as BitwiseShiftOperandParser;
use crate::syntax::tree::expression::tree::builder::Builder as ExpressionTreeBuilder;
use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;

#[derive(Debug, Clone, Copy)]
pub enum State {
    BitwiseShiftOperand,
    BitwiseShiftOperator,
}

impl Default for State {
    fn default() -> Self {
        State::BitwiseShiftOperand
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    next: Option<Token>,
    builder: ExpressionTreeBuilder,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(ExpressionTree, Option<Token>), Error> {
        loop {
            match self.state {
                State::BitwiseShiftOperand => {
                    let (expression, next) = BitwiseShiftOperandParser::default()
                        .parse(stream.clone(), initial.take())?;
                    self.next = next;
                    self.builder.eat(expression);
                    self.state = State::BitwiseShiftOperator;
                }
                State::BitwiseShiftOperator => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
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
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::lexical;
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
    use crate::syntax::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
    use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;

    #[test]
    fn ok() {
        let input = r#"42 << 2"#;

        let expected = Ok((
            ExpressionTree::new(
                Location::new(1, 4),
                ExpressionTreeNode::operator(ExpressionOperator::BitwiseShiftLeft),
                Some(ExpressionTree::new(
                    Location::new(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::new(1, 1),
                            lexical::IntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                    None,
                    None,
                )),
                Some(ExpressionTree::new(
                    Location::new(1, 7),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::new(1, 7),
                            lexical::IntegerLiteral::new_decimal("2".to_owned()),
                        ),
                    )),
                    None,
                    None,
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 8))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
