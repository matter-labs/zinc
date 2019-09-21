//!
//! The comparison operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Location;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::AddSubOperatorOperandParser;
use crate::syntax::OperatorExpression;
use crate::syntax::OperatorExpressionOperator;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    AddSubOperand,
    AddSubOperator,
}

impl Default for State {
    fn default() -> Self {
        State::AddSubOperand
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    expression: OperatorExpression,
    operator: Option<(Location, OperatorExpressionOperator)>,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<OperatorExpression, Error> {
        loop {
            match self.state {
                State::AddSubOperand => {
                    let rpn = AddSubOperatorOperandParser::default().parse(stream.clone())?;
                    self.expression.append(rpn);
                    if let Some((location, operator)) = self.operator.take() {
                        self.expression.push_operator(location, operator);
                    }
                    self.state = State::AddSubOperator;
                }
                State::AddSubOperator => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Plus),
                            location,
                        })) => {
                            stream.borrow_mut().next();
                            self.operator = Some((location, OperatorExpressionOperator::Addition));
                            self.state = State::AddSubOperand;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Minus),
                            location,
                        })) => {
                            stream.borrow_mut().next();
                            self.operator =
                                Some((location, OperatorExpressionOperator::Subtraction));
                            self.state = State::AddSubOperand;
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
        let code = r#"42 + 228 "#;

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
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Addition),
                Token::new(Lexeme::Symbol(Symbol::Plus), Location::new(1, 4)),
            ),
        ]);

        let result = Parser::default()
            .parse(Rc::new(RefCell::new(TokenStream::new(code.to_owned()))))
            .expect("Syntax error");

        assert_eq!(expected, result);
    }
}
