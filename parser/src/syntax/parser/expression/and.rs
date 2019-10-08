//!
//! The logical AND operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Location;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::ComparisonOperandParser;
use crate::syntax::Expression;
use crate::syntax::ExpressionBuilder;
use crate::syntax::ExpressionOperator;
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
    builder: ExpressionBuilder,
    operator: Option<(Location, ExpressionOperator)>,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<Expression, Error> {
        loop {
            match self.state {
                State::ComparisonFirstOperand => {
                    let rpn = ComparisonOperandParser::default().parse(stream.clone())?;
                    self.builder.set_location_if_unset(rpn.location);
                    self.builder.extend_with_expression(rpn);
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
                            self.operator = Some((location, ExpressionOperator::Equal));
                            self.state = State::ComparisonSecondOperand;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::ExclamationMarkEquals),
                            location,
                        })) => {
                            stream.borrow_mut().next();
                            self.operator = Some((location, ExpressionOperator::NotEqual));
                            self.state = State::ComparisonSecondOperand;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::GreaterThanEquals),
                            location,
                        })) => {
                            stream.borrow_mut().next();
                            self.operator = Some((location, ExpressionOperator::GreaterEqual));
                            self.state = State::ComparisonSecondOperand;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::LesserThanEquals),
                            location,
                        })) => {
                            stream.borrow_mut().next();
                            self.operator = Some((location, ExpressionOperator::LesserEqual));
                            self.state = State::ComparisonSecondOperand;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::GreaterThan),
                            location,
                        })) => {
                            stream.borrow_mut().next();
                            self.operator = Some((location, ExpressionOperator::Greater));
                            self.state = State::ComparisonSecondOperand;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::LesserThan),
                            location,
                        })) => {
                            stream.borrow_mut().next();
                            self.operator = Some((location, ExpressionOperator::Lesser));
                            self.state = State::ComparisonSecondOperand;
                        }
                        _ => return Ok(self.builder.finish()),
                    }
                }
                State::ComparisonSecondOperand => {
                    let rpn = ComparisonOperandParser::default().parse(stream.clone())?;
                    self.builder.extend_with_expression(rpn);
                    if let Some((location, operator)) = self.operator.take() {
                        self.builder.push_operator(location, operator);
                    }
                    return Ok(self.builder.finish());
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
    use crate::lexical::BooleanLiteral;
    use crate::lexical::Location;
    use crate::lexical::TokenStream;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::ExpressionOperator;
    use crate::syntax::Literal;

    #[test]
    fn ok() {
        let input = r#"true == false"#;

        let expected = Ok(Expression::new(
            Location::new(1, 1),
            vec![
                ExpressionElement::new(
                    Location::new(1, 1),
                    ExpressionObject::Operand(ExpressionOperand::Literal(Literal::new(
                        Location::new(1, 1),
                        lexical::Literal::Boolean(BooleanLiteral::True),
                    ))),
                ),
                ExpressionElement::new(
                    Location::new(1, 9),
                    ExpressionObject::Operand(ExpressionOperand::Literal(Literal::new(
                        Location::new(1, 9),
                        lexical::Literal::Boolean(BooleanLiteral::False),
                    ))),
                ),
                ExpressionElement::new(
                    Location::new(1, 6),
                    ExpressionObject::Operator(ExpressionOperator::Equal),
                ),
            ],
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }
}
