//!
//! The comparison operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::AddSubOperatorOperandParser;
use crate::syntax::Expression;
use crate::syntax::ExpressionOperator;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    AddSubOperand,
    AddSubOperator,
    End,
}

impl Default for State {
    fn default() -> Self {
        State::AddSubOperand
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    expression: Expression,
    operator: Option<(ExpressionOperator, Token)>,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<Expression, Error> {
        loop {
            match self.state {
                State::AddSubOperand => {
                    let rpn = AddSubOperatorOperandParser::default().parse(stream.clone())?;
                    self.expression.append(rpn);
                    if let Some(operator) = self.operator.take() {
                        self.expression.push_operator(operator);
                    }
                    self.state = State::AddSubOperator;
                }
                State::AddSubOperator => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(
                            token @ Token {
                                lexeme: Lexeme::Symbol(Symbol::Plus),
                                ..
                            },
                        )) => {
                            stream.borrow_mut().next();
                            self.operator = Some((ExpressionOperator::Addition, token));
                            self.state = State::AddSubOperand;
                        }
                        Some(Ok(
                            token @ Token {
                                lexeme: Lexeme::Symbol(Symbol::Minus),
                                ..
                            },
                        )) => {
                            stream.borrow_mut().next();
                            self.operator = Some((ExpressionOperator::Subtraction, token));
                            self.state = State::AddSubOperand;
                        }
                        _ => self.state = State::End,
                    }
                }
                State::End => return Ok(self.expression),
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
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::ExpressionOperator;

    #[test]
    fn ok() {
        let code = br#"42 + 228 "#;

        let expected = Expression::new(vec![
            ExpressionElement::new(
                ExpressionObject::Operand(ExpressionOperand::Literal(Literal::Integer(
                    IntegerLiteral::decimal(b"42".to_vec()),
                ))),
                Token::new(
                    Lexeme::Literal(Literal::Integer(IntegerLiteral::decimal(b"42".to_vec()))),
                    Location::new(1, 1),
                ),
            ),
            ExpressionElement::new(
                ExpressionObject::Operand(ExpressionOperand::Literal(Literal::Integer(
                    IntegerLiteral::decimal(b"228".to_vec()),
                ))),
                Token::new(
                    Lexeme::Literal(Literal::Integer(IntegerLiteral::decimal(b"228".to_vec()))),
                    Location::new(1, 6),
                ),
            ),
            ExpressionElement::new(
                ExpressionObject::Operator(ExpressionOperator::Addition),
                Token::new(Lexeme::Symbol(Symbol::Plus), Location::new(1, 4)),
            ),
        ]);

        let result = Parser::default()
            .parse(Rc::new(RefCell::new(TokenStream::new(code.to_vec()))))
            .expect("Syntax error");

        assert_eq!(expected, result);
    }
}
