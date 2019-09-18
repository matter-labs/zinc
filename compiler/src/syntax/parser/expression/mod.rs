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
    use crate::lexical::BooleanLiteral;
    use crate::lexical::IntegerLiteral;
    use crate::lexical::Lexeme;
    use crate::lexical::Literal;
    use crate::lexical::Location;
    use crate::lexical::Symbol;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::BlockExpression;
    use crate::syntax::Expression;
    use crate::syntax::OperatorExpression;
    use crate::syntax::OperatorExpressionElement;
    use crate::syntax::OperatorExpressionObject;
    use crate::syntax::OperatorExpressionOperand;
    use crate::syntax::OperatorExpressionOperator;

    #[test]
    fn ok() {
        let code = r#"true || { 2 + 1 == 3 }"#;

        let expected = Expression::Operator(OperatorExpression::new(vec![
            OperatorExpressionElement::new(
                OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                    Literal::Boolean(BooleanLiteral::True),
                )),
                Token::new(
                    Lexeme::Literal(Literal::Boolean(BooleanLiteral::True)),
                    Location::new(1, 1),
                ),
            ),
            OperatorExpressionElement::new(
                OperatorExpressionObject::Operand(OperatorExpressionOperand::Block(
                    BlockExpression::new(
                        Location::new(1, 9),
                        vec![],
                        Some(Expression::Operator(OperatorExpression::new(vec![
                            OperatorExpressionElement::new(
                                OperatorExpressionObject::Operand(
                                    OperatorExpressionOperand::Literal(Literal::Integer(
                                        IntegerLiteral::decimal("2".to_owned()),
                                    )),
                                ),
                                Token::new(
                                    Lexeme::Literal(Literal::Integer(IntegerLiteral::decimal(
                                        "2".to_owned(),
                                    ))),
                                    Location::new(1, 11),
                                ),
                            ),
                            OperatorExpressionElement::new(
                                OperatorExpressionObject::Operand(
                                    OperatorExpressionOperand::Literal(Literal::Integer(
                                        IntegerLiteral::decimal("1".to_owned()),
                                    )),
                                ),
                                Token::new(
                                    Lexeme::Literal(Literal::Integer(IntegerLiteral::decimal(
                                        "1".to_owned(),
                                    ))),
                                    Location::new(1, 15),
                                ),
                            ),
                            OperatorExpressionElement::new(
                                OperatorExpressionObject::Operator(
                                    OperatorExpressionOperator::Addition,
                                ),
                                Token::new(Lexeme::Symbol(Symbol::Plus), Location::new(1, 13)),
                            ),
                            OperatorExpressionElement::new(
                                OperatorExpressionObject::Operand(
                                    OperatorExpressionOperand::Literal(Literal::Integer(
                                        IntegerLiteral::decimal("3".to_owned()),
                                    )),
                                ),
                                Token::new(
                                    Lexeme::Literal(Literal::Integer(IntegerLiteral::decimal(
                                        "3".to_owned(),
                                    ))),
                                    Location::new(1, 20),
                                ),
                            ),
                            OperatorExpressionElement::new(
                                OperatorExpressionObject::Operator(
                                    OperatorExpressionOperator::Equal,
                                ),
                                Token::new(
                                    Lexeme::Symbol(Symbol::DoubleEquals),
                                    Location::new(1, 17),
                                ),
                            ),
                        ]))),
                    ),
                )),
                Token::new(
                    Lexeme::Symbol(Symbol::BracketCurlyLeft),
                    Location::new(1, 9),
                ),
            ),
            OperatorExpressionElement::new(
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Or),
                Token::new(
                    Lexeme::Symbol(Symbol::DoubleVerticalBar),
                    Location::new(1, 6),
                ),
            ),
        ]));

        let result = Parser::default()
            .parse(Rc::new(RefCell::new(TokenStream::new(code.to_owned()))))
            .expect("Syntax error");

        assert_eq!(expected, result);
    }
}
