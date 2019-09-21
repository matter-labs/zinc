//!
//! The AND operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Location;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::ComparisonOperatorOperandParser;
use crate::syntax::OperatorExpression;
use crate::syntax::OperatorExpressionOperator;
use crate::Error;

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
    expression: OperatorExpression,
    operator: Option<(Location, OperatorExpressionOperator)>,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<OperatorExpression, Error> {
        loop {
            match self.state {
                State::ComparisonFirstOperand => {
                    let rpn = ComparisonOperatorOperandParser::default().parse(stream.clone())?;
                    self.expression.append(rpn);
                    self.state = State::ComparisonOperator;
                }
                State::ComparisonOperator => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleEquals),
                            location,
                        })) => {
                            stream.borrow_mut().next();
                            self.operator = Some((location, OperatorExpressionOperator::Equal));
                            self.state = State::ComparisonSecondOperand;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::ExclamationMarkEquals),
                            location,
                        })) => {
                            stream.borrow_mut().next();
                            self.operator = Some((location, OperatorExpressionOperator::NotEqual));
                            self.state = State::ComparisonSecondOperand;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::GreaterThanEquals),
                            location,
                        })) => {
                            stream.borrow_mut().next();
                            self.operator =
                                Some((location, OperatorExpressionOperator::GreaterEqual));
                            self.state = State::ComparisonSecondOperand;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::LesserThanEquals),
                            location,
                        })) => {
                            stream.borrow_mut().next();
                            self.operator =
                                Some((location, OperatorExpressionOperator::LesserEqual));
                            self.state = State::ComparisonSecondOperand;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::GreaterThan),
                            location,
                        })) => {
                            stream.borrow_mut().next();
                            self.operator = Some((location, OperatorExpressionOperator::Greater));
                            self.state = State::ComparisonSecondOperand;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::LesserThan),
                            location,
                        })) => {
                            stream.borrow_mut().next();
                            self.operator = Some((location, OperatorExpressionOperator::Lesser));
                            self.state = State::ComparisonSecondOperand;
                        }
                        _ => return Ok(self.expression),
                    }
                }
                State::ComparisonSecondOperand => {
                    let rpn = ComparisonOperatorOperandParser::default().parse(stream.clone())?;
                    self.expression.append(rpn);
                    if let Some((location, operator)) = self.operator.take() {
                        self.expression.push_operator(location, operator);
                    }
                    return Ok(self.expression);
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
    use crate::lexical::BooleanLiteral;
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
        let code = r#"true == false"#;

        let expected = OperatorExpression::new(vec![
            OperatorExpressionElement::new(
                OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                    Literal::Boolean(BooleanLiteral::True),
                )),
                Token::new(
                    Lexeme::Literal(Literal::Boolean(BooleanLiteral::True)),
                    Location::new(1, 1),
                ),
            ),
            OperatorExpressionElement::new(
                OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                    Literal::Boolean(BooleanLiteral::False),
                )),
                Token::new(
                    Lexeme::Literal(Literal::Boolean(BooleanLiteral::False)),
                    Location::new(1, 9),
                ),
            ),
            OperatorExpressionElement::new(
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Equal),
                Token::new(Lexeme::Symbol(Symbol::DoubleEquals), Location::new(1, 6)),
            ),
        ]);

        let result = Parser::default()
            .parse(Rc::new(RefCell::new(TokenStream::new(code.to_owned()))))
            .expect("Syntax error");

        assert_eq!(expected, result);
    }
}
