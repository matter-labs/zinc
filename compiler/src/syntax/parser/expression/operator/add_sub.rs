//!
//! The addition/subtraction operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::MulDivRemOperatorOperandParser;
use crate::syntax::OperatorExpression;
use crate::syntax::OperatorExpressionOperator;
use crate::Error;

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
    expression: OperatorExpression,
    operator: Option<(OperatorExpressionOperator, Token)>,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<OperatorExpression, Error> {
        loop {
            match self.state {
                State::MulDivRemOperand => {
                    let rpn = MulDivRemOperatorOperandParser::default().parse(stream.clone())?;
                    self.expression.append(rpn);
                    if let Some(operator) = self.operator.take() {
                        self.expression.push_operator(operator);
                    }
                    self.state = State::MulDivRemOperator;
                }
                State::MulDivRemOperator => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(
                            token @ Token {
                                lexeme: Lexeme::Symbol(Symbol::Asterisk),
                                ..
                            },
                        )) => {
                            stream.borrow_mut().next();
                            self.operator =
                                Some((OperatorExpressionOperator::Multiplication, token));
                            self.state = State::MulDivRemOperand;
                        }
                        Some(Ok(
                            token @ Token {
                                lexeme: Lexeme::Symbol(Symbol::Slash),
                                ..
                            },
                        )) => {
                            stream.borrow_mut().next();
                            self.operator = Some((OperatorExpressionOperator::Division, token));
                            self.state = State::MulDivRemOperand;
                        }
                        Some(Ok(
                            token @ Token {
                                lexeme: Lexeme::Symbol(Symbol::Percent),
                                ..
                            },
                        )) => {
                            stream.borrow_mut().next();
                            self.operator = Some((OperatorExpressionOperator::Remainder, token));
                            self.state = State::MulDivRemOperand;
                        }
                        _ => return Ok(self.expression),
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
    use crate::lexical::IntegerLiteral;
    use crate::lexical::Lexeme;
    use crate::lexical::Literal;
    use crate::lexical::Location;
    use crate::lexical::Symbol;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::OperatorExpression;
    use crate::syntax::OperatorExpressionElement;
    use crate::syntax::OperatorExpressionObject;
    use crate::syntax::OperatorExpressionOperand;
    use crate::syntax::OperatorExpressionOperator;

    #[test]
    fn ok() {
        let code = r#"42 * 228 "#;

        let expected = OperatorExpression::new(vec![
            OperatorExpressionElement::new(
                OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                    Literal::Integer(IntegerLiteral::decimal("42".to_owned())),
                )),
                Token::new(
                    Lexeme::Literal(Literal::Integer(IntegerLiteral::decimal("42".to_owned()))),
                    Location::new(1, 1),
                ),
            ),
            OperatorExpressionElement::new(
                OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                    Literal::Integer(IntegerLiteral::decimal("228".to_owned())),
                )),
                Token::new(
                    Lexeme::Literal(Literal::Integer(IntegerLiteral::decimal("228".to_owned()))),
                    Location::new(1, 6),
                ),
            ),
            OperatorExpressionElement::new(
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Multiplication),
                Token::new(Lexeme::Symbol(Symbol::Asterisk), Location::new(1, 4)),
            ),
        ]);

        let result = Parser::default()
            .parse(Rc::new(RefCell::new(TokenStream::new(code.to_owned()))))
            .expect("Syntax error");

        assert_eq!(expected, result);
    }
}
