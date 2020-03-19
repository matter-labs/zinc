//!
//! The expression parser.
//!

pub mod access;
pub mod add_sub;
pub mod and;
pub mod array;
pub mod assignment;
pub mod bitwise_and;
pub mod bitwise_or;
pub mod bitwise_shift;
pub mod bitwise_xor;
pub mod block;
pub mod casting;
pub mod comparison;
pub mod conditional;
pub mod list;
pub mod r#match;
pub mod mul_div_rem;
pub mod or;
pub mod path;
pub mod range;
pub mod structure;
pub mod terminal;
pub mod tuple;
pub mod xor;

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Lexeme;
use crate::lexical::Location;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::parser::expression::assignment::Parser as AssignmentOperandParser;
use crate::syntax::tree::expression::builder::Builder as ExpressionBuilder;
use crate::syntax::tree::expression::operator::Operator as ExpressionOperator;
use crate::syntax::tree::expression::Expression;

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
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Equals),
                            location,
                        } => {
                            self.operator = Some((location, ExpressionOperator::Assignment));
                            self.state = State::AssignmentSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::PlusEquals),
                            location,
                        } => {
                            self.operator =
                                Some((location, ExpressionOperator::AssignmentAddition));
                            self.state = State::AssignmentSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::MinusEquals),
                            location,
                        } => {
                            self.operator =
                                Some((location, ExpressionOperator::AssignmentSubtraction));
                            self.state = State::AssignmentSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::AsteriskEquals),
                            location,
                        } => {
                            self.operator =
                                Some((location, ExpressionOperator::AssignmentMultiplication));
                            self.state = State::AssignmentSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::SlashEquals),
                            location,
                        } => {
                            self.operator =
                                Some((location, ExpressionOperator::AssignmentDivision));
                            self.state = State::AssignmentSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::PercentEquals),
                            location,
                        } => {
                            self.operator =
                                Some((location, ExpressionOperator::AssignmentRemainder));
                            self.state = State::AssignmentSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::VerticalBarEquals),
                            location,
                        } => {
                            self.operator =
                                Some((location, ExpressionOperator::AssignmentBitwiseOr));
                            self.state = State::AssignmentSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::CircumflexEquals),
                            location,
                        } => {
                            self.operator =
                                Some((location, ExpressionOperator::AssignmentBitwiseXor));
                            self.state = State::AssignmentSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::AmpersandEquals),
                            location,
                        } => {
                            self.operator =
                                Some((location, ExpressionOperator::AssignmentBitwiseAnd));
                            self.state = State::AssignmentSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleLesserEquals),
                            location,
                        } => {
                            self.operator =
                                Some((location, ExpressionOperator::AssignmentBitwiseShiftLeft));
                            self.state = State::AssignmentSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleGreaterEquals),
                            location,
                        } => {
                            self.operator =
                                Some((location, ExpressionOperator::AssignmentBitwiseShiftRight));
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
    use crate::syntax::tree::expression::element::Element as ExpressionElement;
    use crate::syntax::tree::expression::object::Object as ExpressionObject;
    use crate::syntax::tree::expression::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::operator::Operator as ExpressionOperator;
    use crate::syntax::tree::expression::Expression;
    use crate::syntax::tree::literal::boolean::Literal as BooleanLiteral;

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

        assert_eq!(result, expected);
    }
}
