//!
//! The expression parser.
//!

mod block;
mod operator;

pub use self::block::Parser as BlockExpressionParser;
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
    use crate::lexical::Lexeme;
    use crate::lexical::Literal;
    use crate::lexical::Location;
    use crate::lexical::Symbol;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::Expression;
    use crate::syntax::OperatorExpression;
    use crate::syntax::OperatorExpressionElement;
    use crate::syntax::OperatorExpressionObject;
    use crate::syntax::OperatorExpressionOperand;
    use crate::syntax::OperatorExpressionOperator;

    #[test]
    fn ok() {
        let code = br#"true || false"#;

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
                OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                    Literal::Boolean(BooleanLiteral::False),
                )),
                Token::new(
                    Lexeme::Literal(Literal::Boolean(BooleanLiteral::False)),
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
            .parse(Rc::new(RefCell::new(TokenStream::new(code.to_vec()))))
            .expect("Syntax error");

        assert_eq!(expected, result);
    }
}
