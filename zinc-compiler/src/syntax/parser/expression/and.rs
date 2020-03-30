//!
//! The logical AND operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::parser::expression::comparison::Parser as ComparisonOperandParser;
use crate::syntax::tree::expression::tree::builder::Builder as ExpressionTreeBuilder;
use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;

#[derive(Debug, Clone, Copy)]
pub enum State {
    ComparisonFirstOperand,
    ComparisonOperator,
    ComparisonSecondOperand,
}

impl Default for State {
    fn default() -> Self {
        State::ComparisonFirstOperand
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
                State::ComparisonFirstOperand => {
                    let (expression, next) =
                        ComparisonOperandParser::default().parse(stream.clone(), initial.take())?;
                    self.next = next;
                    self.builder.eat(expression);
                    self.state = State::ComparisonOperator;
                }
                State::ComparisonOperator => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleEquals),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::Equals, location);
                            self.state = State::ComparisonSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ExclamationMarkEquals),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::NotEquals, location);
                            self.state = State::ComparisonSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::GreaterEquals),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::GreaterEquals, location);
                            self.state = State::ComparisonSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::LesserEquals),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::LesserEquals, location);
                            self.state = State::ComparisonSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Greater),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::Greater, location);
                            self.state = State::ComparisonSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Lesser),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::Lesser, location);
                            self.state = State::ComparisonSecondOperand;
                        }
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
                State::ComparisonSecondOperand => {
                    let (expression, next) =
                        ComparisonOperandParser::default().parse(stream, None)?;
                    self.builder.eat(expression);
                    return Ok((self.builder.finish(), next));
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
    use crate::syntax::tree::literal::boolean::Literal as BooleanLiteral;

    #[test]
    fn ok() {
        let input = r#"true == false"#;

        let expected = Ok((
            ExpressionTree::new(
                Location::new(1, 6),
                ExpressionTreeNode::operator(ExpressionOperator::Equals),
                Some(ExpressionTree::new(
                    Location::new(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                        BooleanLiteral::new(Location::new(1, 1), lexical::BooleanLiteral::r#true()),
                    )),
                    None,
                    None,
                )),
                Some(ExpressionTree::new(
                    Location::new(1, 9),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                        BooleanLiteral::new(
                            Location::new(1, 9),
                            lexical::BooleanLiteral::r#false(),
                        ),
                    )),
                    None,
                    None,
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 14))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
