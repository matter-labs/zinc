//!
//! The assignment operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Location;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Expression;
use crate::syntax::ExpressionBuilder;
use crate::syntax::ExpressionOperator;
use crate::syntax::OrOperandParser;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    LogicalOrOperand,
    LogicalOrOperator,
}

impl Default for State {
    fn default() -> Self {
        State::LogicalOrOperand
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
                State::LogicalOrOperand => {
                    let rpn = OrOperandParser::default().parse(stream.clone())?;
                    self.builder.set_location_if_unset(rpn.location);
                    self.builder.extend_with_expression(rpn);
                    if let Some((location, operator)) = self.operator.take() {
                        self.builder.push_operator(location, operator);
                    }
                    self.state = State::LogicalOrOperator;
                }
                State::LogicalOrOperator => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleVerticalBar),
                            location,
                        })) => {
                            stream.borrow_mut().next();
                            self.operator = Some((location, ExpressionOperator::Or));
                            self.state = State::LogicalOrOperand;
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
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::ExpressionOperator;
    use crate::syntax::Literal;

    #[test]
    fn ok() {
        let input = r#"true || false"#;

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
                    ExpressionObject::Operator(ExpressionOperator::Or),
                ),
            ],
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }
}
