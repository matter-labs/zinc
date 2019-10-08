//!
//! The expression parser.
//!

mod access;
mod add_sub;
mod and;
mod array;
mod assignment;
mod block;
mod casting;
mod comparison;
mod conditional;
mod mul_div_rem;
mod or;
mod structure;
mod tuple;
mod xor;

pub use self::access::Parser as AccessOperandParser;
pub use self::add_sub::Parser as AddSubOperandParser;
pub use self::and::Parser as AndOperandParser;
pub use self::array::Parser as ArrayExpressionParser;
pub use self::assignment::Parser as AssignmentOperandParser;
pub use self::block::Parser as BlockExpressionParser;
pub use self::casting::Parser as CastingOperandParser;
pub use self::comparison::Parser as ComparisonOperandParser;
pub use self::conditional::Parser as ConditionalExpressionParser;
pub use self::mul_div_rem::Parser as MulDivRemOperandParser;
pub use self::or::Parser as OrOperandParser;
pub use self::structure::Parser as StructureExpressionParser;
pub use self::tuple::Parser as TupleExpressionParser;
pub use self::xor::Parser as XorOperandParser;

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
    builder: ExpressionBuilder,
    operator: Option<(Location, ExpressionOperator)>,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<Expression, Error> {
        loop {
            match self.state {
                State::AssignmentFirstOperand => {
                    let rpn = AssignmentOperandParser::default().parse(stream.clone())?;
                    self.builder.set_location_if_unset(rpn.location);
                    self.builder.extend_with_expression(rpn);
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
                            self.operator = Some((location, ExpressionOperator::Assignment));
                            self.state = State::AssignmentSecondOperand;
                        }
                        _ => return Ok(self.builder.finish()),
                    }
                }
                State::AssignmentSecondOperand => {
                    let rpn = AssignmentOperandParser::default().parse(stream.clone())?;
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
