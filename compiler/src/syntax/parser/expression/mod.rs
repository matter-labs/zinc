//!
//! The expression parser.
//!

mod block;
mod conditional;
mod operator;

pub use self::block::Parser as BlockExpressionParser;
pub use self::conditional::Parser as ConditionalExpressionParser;
pub use self::operator::AddSubOperandParser as AddSubOperatorOperandParser;
pub use self::operator::AndOperandParser as AndOperatorOperandParser;
pub use self::operator::AssignmentOperandParser as AssignmentOperatorOperandParser;
pub use self::operator::CastingOperandParser as CastingOperatorOperandParser;
pub use self::operator::ComparisonOperandParser as ComparisonOperatorOperandParser;
pub use self::operator::MulDivRemOperandParser as MulDivRemOperatorOperandParser;
pub use self::operator::OrOperandParser as OrOperatorOperandParser;
pub use self::operator::Parser as OperatorExpressionParser;
pub use self::operator::XorOperandParser as XorOperatorOperandParser;

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Expression;
use crate::Error;

#[derive(Default)]
pub struct Parser {}

impl Parser {
    pub fn parse(self, stream: Rc<RefCell<TokenStream>>) -> Result<Expression, Error> {
        let peek = stream.borrow_mut().peek();
        match peek {
            Some(Ok(Token {
                lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                ..
            })) => BlockExpressionParser::default()
                .parse(stream)
                .map(Expression::Block),
            Some(Ok(Token {
                lexeme: Lexeme::Keyword(Keyword::If),
                ..
            })) => ConditionalExpressionParser::default()
                .parse(stream)
                .map(Expression::Conditional),
            Some(Ok(..)) => OperatorExpressionParser::default()
                .parse(stream)
                .map(Expression::Operator),
            Some(Err(error)) => Err(Error::Lexical(error)),
            None => Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
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
    use crate::lexical::IntegerLiteral;
    use crate::lexical::Location;
    use crate::lexical::TokenStream;
    use crate::syntax::BlockExpression;
    use crate::syntax::Expression;
    use crate::syntax::Literal;
    use crate::syntax::OperatorExpression;
    use crate::syntax::OperatorExpressionElement;
    use crate::syntax::OperatorExpressionObject;
    use crate::syntax::OperatorExpressionOperand;
    use crate::syntax::OperatorExpressionOperator;

    #[test]
    fn ok() {
        let code = r#"true || { 2 + 1 == 3 }"#;

        let expected = Expression::Operator(OperatorExpression::new(
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
                    OperatorExpressionObject::Operand(OperatorExpressionOperand::Block(
                        BlockExpression::new(
                            Location::new(1, 9),
                            vec![],
                            Some(Expression::Operator(OperatorExpression::new(
                                Location::new(1, 11),
                                vec![
                                    OperatorExpressionElement::new(
                                        Location::new(1, 11),
                                        OperatorExpressionObject::Operand(
                                            OperatorExpressionOperand::Literal(Literal::new(
                                                Location::new(1, 11),
                                                lexical::Literal::Integer(IntegerLiteral::decimal(
                                                    "2".to_owned(),
                                                )),
                                            )),
                                        ),
                                    ),
                                    OperatorExpressionElement::new(
                                        Location::new(1, 15),
                                        OperatorExpressionObject::Operand(
                                            OperatorExpressionOperand::Literal(Literal::new(
                                                Location::new(1, 15),
                                                lexical::Literal::Integer(IntegerLiteral::decimal(
                                                    "1".to_owned(),
                                                )),
                                            )),
                                        ),
                                    ),
                                    OperatorExpressionElement::new(
                                        Location::new(1, 13),
                                        OperatorExpressionObject::Operator(
                                            OperatorExpressionOperator::Addition,
                                        ),
                                    ),
                                    OperatorExpressionElement::new(
                                        Location::new(1, 20),
                                        OperatorExpressionObject::Operand(
                                            OperatorExpressionOperand::Literal(Literal::new(
                                                Location::new(1, 20),
                                                lexical::Literal::Integer(IntegerLiteral::decimal(
                                                    "3".to_owned(),
                                                )),
                                            )),
                                        ),
                                    ),
                                    OperatorExpressionElement::new(
                                        Location::new(1, 17),
                                        OperatorExpressionObject::Operator(
                                            OperatorExpressionOperator::Equal,
                                        ),
                                    ),
                                ],
                            ))),
                        ),
                    )),
                ),
                OperatorExpressionElement::new(
                    Location::new(1, 6),
                    OperatorExpressionObject::Operator(OperatorExpressionOperator::Or),
                ),
            ],
        ));

        let result = Parser::default()
            .parse(Rc::new(RefCell::new(TokenStream::new(code.to_owned()))))
            .expect("Syntax error");

        assert_eq!(expected, result);
    }
}
