//!
//! The operator expression parser.
//!

mod add_sub;
mod and;
mod assignment;
mod casting;
mod comparison;
mod indexing;
mod mul_div_rem;
mod or;
mod xor;

pub use self::add_sub::Parser as AddSubOperandParser;
pub use self::and::Parser as AndOperandParser;
pub use self::assignment::Parser as AssignmentOperandParser;
pub use self::casting::Parser as CastingOperandParser;
pub use self::comparison::Parser as ComparisonOperandParser;
pub use self::indexing::Parser as IndexingOperandParser;
pub use self::mul_div_rem::Parser as MulDivRemOperandParser;
pub use self::or::Parser as OrOperandParser;
pub use self::xor::Parser as XorOperandParser;

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
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    AssignmentFirstOperand,
    AssignmentOperator,
    AssignmentSecondOperand,
}

impl Default for State {
    fn default() -> Self {
        State::AssignmentFirstOperand
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
                State::AssignmentFirstOperand => {
                    let rpn = AssignmentOperandParser::default().parse(stream.clone())?;
                    self.builder.set_location_if_unset(rpn.location);
                    self.builder.append_expression(rpn);
                    self.state = State::AssignmentOperator;
                }
                State::AssignmentOperator => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Equals),
                            location,
                        })) => {
                            stream.borrow_mut().next();
                            self.operator =
                                Some((location, OperatorExpressionOperator::Assignment));
                            self.state = State::AssignmentSecondOperand;
                        }
                        _ => return Ok(self.builder.finish()),
                    }
                }
                State::AssignmentSecondOperand => {
                    let rpn = AssignmentOperandParser::default().parse(stream.clone())?;
                    self.builder.append_expression(rpn);
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
    use crate::syntax::Literal;
    use crate::syntax::OperatorExpression;
    use crate::syntax::OperatorExpressionElement;
    use crate::syntax::OperatorExpressionObject;
    use crate::syntax::OperatorExpressionOperand;
    use crate::syntax::OperatorExpressionOperator;

    #[test]
    fn ok() {
        let code = r#"true || false"#;

        let expected = OperatorExpression::new(
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
                    OperatorExpressionObject::Operator(OperatorExpressionOperator::Or),
                ),
            ],
        );

        let result = Parser::default()
            .parse(Rc::new(RefCell::new(TokenStream::new(code.to_owned()))))
            .expect("Syntax error");

        assert_eq!(expected, result);
    }
}
