//!
//! The logical OR operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::parser::expression::xor::Parser as XorOperandParser;
use crate::syntax::tree::expression::tree::builder::Builder as ExpressionTreeBuilder;
use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;

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
                State::LogicalXorOperand => {
                    let (expression, next) =
                        XorOperandParser::default().parse(stream.clone(), initial.take())?;
                    self.next = next;
                    self.builder.eat(expression);
                    self.state = State::LogicalXorOperator;
                }
                State::LogicalXorOperator => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleCircumflex),
                            location,
                        } => {
                            self.builder.eat_operator(ExpressionOperator::Xor, location);
                            self.state = State::LogicalXorOperand;
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
//     use crate::syntax::tree::literal::boolean::Literal as BooleanLiteral;
//
//     #[test]
//     fn ok() {
//         let input = r#"true ^^ false"#;
//
//         let expected = Ok((
//             Expression::new(
//                 Location::new(1, 1),
//                 vec![
//                     ExpressionElement::new(
//                         Location::new(1, 1),
//                         ExpressionObject::Operand(ExpressionOperand::LiteralBoolean(
//                             BooleanLiteral::new(Location::new(1, 1), lexical::BooleanLiteral::True),
//                         )),
//                     ),
//                     ExpressionElement::new(
//                         Location::new(1, 9),
//                         ExpressionObject::Operand(ExpressionOperand::LiteralBoolean(
//                             BooleanLiteral::new(
//                                 Location::new(1, 9),
//                                 lexical::BooleanLiteral::False,
//                             ),
//                         )),
//                     ),
//                     ExpressionElement::new(
//                         Location::new(1, 6),
//                         ExpressionObject::Operator(ExpressionOperator::Xor),
//                     ),
//                 ],
//             ),
//             Some(Token::new(Lexeme::Eof, Location::new(1, 14))),
//         ));
//
//         let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);
//
//         assert_eq!(result, expected);
//     }
// }
