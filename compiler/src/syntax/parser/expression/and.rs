//!
//! The logical AND operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Lexeme;
use crate::lexical::Location;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::ComparisonOperandParser;
use crate::syntax::Expression;
use crate::syntax::ExpressionBuilder;
use crate::syntax::ExpressionOperator;

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
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(Expression, Option<Token>), Error> {
        loop {
            match self.state {
                State::ComparisonFirstOperand => {
                    let (expression, next) =
                        ComparisonOperandParser::default().parse(stream.clone(), initial.take())?;
                    self.next = next;
                    self.builder.set_location_if_unset(expression.location);
                    self.builder.extend_with_expression(expression);
                    self.state = State::ComparisonOperator;
                }
                State::ComparisonOperator => {
                    match self.next.take().expect("Always contains a value") {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleEquals),
                            location,
                        } => {
                            self.operator = Some((location, ExpressionOperator::Equals));
                            self.state = State::ComparisonSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ExclamationMarkEquals),
                            location,
                        } => {
                            self.operator = Some((location, ExpressionOperator::NotEquals));
                            self.state = State::ComparisonSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::GreaterThanEquals),
                            location,
                        } => {
                            self.operator = Some((location, ExpressionOperator::GreaterEquals));
                            self.state = State::ComparisonSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::LesserThanEquals),
                            location,
                        } => {
                            self.operator = Some((location, ExpressionOperator::LesserEquals));
                            self.state = State::ComparisonSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::GreaterThan),
                            location,
                        } => {
                            self.operator = Some((location, ExpressionOperator::Greater));
                            self.state = State::ComparisonSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::LesserThan),
                            location,
                        } => {
                            self.operator = Some((location, ExpressionOperator::Lesser));
                            self.state = State::ComparisonSecondOperand;
                        }
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
                State::ComparisonSecondOperand => {
                    let (expression, next) =
                        ComparisonOperandParser::default().parse(stream.clone(), None)?;
                    self.builder.extend_with_expression(expression);
                    if let Some((location, operator)) = self.operator.take() {
                        self.builder.push_operator(location, operator);
                    }
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
    use crate::lexical::BooleanLiteral;
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Token;
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

        let expected = Ok((
            Expression::new(
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
                        ExpressionObject::Operator(ExpressionOperator::Equals),
                    ),
                ],
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 14))),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }
}
