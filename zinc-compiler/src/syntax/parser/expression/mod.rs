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
mod list;
mod r#match;
mod mul_div_rem;
mod or;
mod path;
mod range;
mod structure;
mod terminal;
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
pub use self::list::Parser as ListParser;
pub use self::mul_div_rem::Parser as MulDivRemOperandParser;
pub use self::or::Parser as OrOperandParser;
pub use self::path::Parser as PathOperandParser;
pub use self::r#match::Parser as MatchExpressionParser;
pub use self::range::Parser as RangeOperandParser;
pub use self::structure::Parser as StructureExpressionParser;
pub use self::terminal::Parser as TerminalOperandParser;
pub use self::tuple::Parser as TupleExpressionParser;
pub use self::xor::Parser as XorOperandParser;

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Lexeme;
use crate::lexical::Location;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Expression;
use crate::syntax::ExpressionBuilder;
use crate::syntax::ExpressionOperator;

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
                State::AssignmentFirstOperand => {
                    let (expression, next) =
                        AssignmentOperandParser::default().parse(stream.clone(), initial.take())?;
                    self.next = next;
                    self.builder.set_location_if_unset(expression.location);
                    self.builder.extend_with_expression(expression);
                    self.state = State::AssignmentOperator;
                }
                State::AssignmentOperator => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Equals),
                            location,
                        } => {
                            self.operator = Some((location, ExpressionOperator::Assignment));
                            self.state = State::AssignmentSecondOperand;
                        }
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
                State::AssignmentSecondOperand => {
                    let (expression, token) =
                        AssignmentOperandParser::default().parse(stream, None)?;
                    self.builder.extend_with_expression(expression);
                    if let Some((location, operator)) = self.operator.take() {
                        self.builder.push_operator(location, operator);
                    }
                    return Ok((self.builder.finish(), token));
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
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::BooleanLiteral;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::ExpressionOperator;

    #[test]
    fn ok() {
        let input = r#"true || false"#;

        let expected = Ok((
            Expression::new(
                Location::new(1, 1),
                vec![
                    ExpressionElement::new(
                        Location::new(1, 1),
                        ExpressionObject::Operand(ExpressionOperand::LiteralBoolean(
                            BooleanLiteral::new(Location::new(1, 1), lexical::BooleanLiteral::True),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 9),
                        ExpressionObject::Operand(ExpressionOperand::LiteralBoolean(
                            BooleanLiteral::new(
                                Location::new(1, 9),
                                lexical::BooleanLiteral::False,
                            ),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 6),
                        ExpressionObject::Operator(ExpressionOperator::Or),
                    ),
                ],
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 14))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(expected, result);
    }
}
