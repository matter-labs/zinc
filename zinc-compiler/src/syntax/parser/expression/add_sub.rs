//!
//! The addition/subtraction operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::parser::expression::mul_div_rem::Parser as MulDivRemOperandParser;
use crate::syntax::tree::expression::tree::builder::Builder as ExpressionTreeBuilder;
use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;

#[derive(Debug, Clone, Copy)]
pub enum State {
    MulDivRemOperand,
    MulDivRemOperator,
}

impl Default for State {
    fn default() -> Self {
        State::MulDivRemOperand
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    next: Option<Token>,
    builder: ExpressionTreeBuilder,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(ExpressionTree, Option<Token>), Error> {
        loop {
            match self.state {
                State::MulDivRemOperand => {
                    let (expression, next) =
                        MulDivRemOperandParser::default().parse(stream.clone(), initial.take())?;
                    self.next = next;
                    self.builder.eat(expression);
                    self.state = State::MulDivRemOperator;
                }
                State::MulDivRemOperator => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Asterisk),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::Multiplication, location);
                            self.state = State::MulDivRemOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Slash),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::Division, location);
                            self.state = State::MulDivRemOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Percent),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::Remainder, location);
                            self.state = State::MulDivRemOperand;
                        }
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use std::cell::RefCell;
//     use std::rc::Rc;
//
//     use super::Parser;
//     use crate::lexical;
//     use crate::lexical::Lexeme;
//     use crate::lexical::Location;
//     use crate::lexical::Token;
//     use crate::lexical::TokenStream;
//     use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
//     use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
//     use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
//
//     #[test]
//     fn ok() {
//         let input = r#"42 * 228"#;
//
//         let expected = Ok((
//             Expression::new(
//                 Location::new(1, 1),
//                 vec![
//                     ExpressionElement::new(
//                         Location::new(1, 1),
//                         ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
//                             IntegerLiteral::new(
//                                 Location::new(1, 1),
//                                 lexical::IntegerLiteral::new_decimal("42".to_owned()),
//                             ),
//                         )),
//                     ),
//                     ExpressionElement::new(
//                         Location::new(1, 6),
//                         ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
//                             IntegerLiteral::new(
//                                 Location::new(1, 6),
//                                 lexical::IntegerLiteral::new_decimal("228".to_owned()),
//                             ),
//                         )),
//                     ),
//                     ExpressionElement::new(
//                         Location::new(1, 4),
//                         ExpressionObject::Operator(ExpressionOperator::Multiplication),
//                     ),
//                 ],
//             ),
//             Some(Token::new(Lexeme::Eof, Location::new(1, 9))),
//         ));
//
//         let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);
//
//         assert_eq!(result, expected);
//     }
// }
