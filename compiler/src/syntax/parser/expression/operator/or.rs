//!
//! The OR operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Location;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::OperatorExpression;
use crate::syntax::OperatorExpressionBuilder;
use crate::syntax::OperatorExpressionOperator;
use crate::syntax::XorOperatorOperandParser;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    LogicalXorOperand,
    LogicalXorOperator,
}

impl Default for State {
    fn default() -> Self {
        State::LogicalXorOperand
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: OperatorExpressionBuilder,
    operator: Option<(Location, OperatorExpressionOperator)>,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<OperatorExpression, Error> {
        loop {
            match self.state {
                State::LogicalXorOperand => {
                    let rpn = XorOperatorOperandParser::default().parse(stream.clone())?;
                    self.builder.set_location_if_unset(rpn.location);
                    self.builder.extend_with_expression(rpn);
                    if let Some((location, operator)) = self.operator.take() {
                        self.builder.push_operator(location, operator);
                    }
                    self.state = State::LogicalXorOperator;
                }
                State::LogicalXorOperator => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleCircumflex),
                            location,
                        })) => {
                            stream.borrow_mut().next();
                            self.operator = Some((location, OperatorExpressionOperator::Xor));
                            self.state = State::LogicalXorOperand;
                        }
                        _ => return Ok(self.builder.finish()),
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
    use crate::lexical::BooleanLiteral;
    use crate::lexical::Location;
    use crate::lexical::TokenStream;
    use crate::syntax::Literal;
    use crate::syntax::OperatorExpression;
    use crate::syntax::OperatorExpressionElement;
    use crate::syntax::OperatorExpressionObject;
    use crate::syntax::OperatorExpressionOperand;
    use crate::syntax::OperatorExpressionOperator;

    #[test]
    fn ok() {
        let code = r#"true ^^ false"#;

        let expected = Ok(OperatorExpression::new(
            Location::new(1, 1),
            vec![
                OperatorExpressionElement::new(
                    Location::new(1, 1),
                    OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                        Literal::new(
                            Location::new(1, 1),
                            lexical::Literal::Boolean(BooleanLiteral::True),
                        ),
                    )),
                ),
                OperatorExpressionElement::new(
                    Location::new(1, 9),
                    OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                        Literal::new(
                            Location::new(1, 9),
                            lexical::Literal::Boolean(BooleanLiteral::False),
                        ),
                    )),
                ),
                OperatorExpressionElement::new(
                    Location::new(1, 6),
                    OperatorExpressionObject::Operator(OperatorExpressionOperator::Xor),
                ),
            ],
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(code.to_owned()))));

        assert_eq!(expected, result);
    }
}
