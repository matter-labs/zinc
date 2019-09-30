//!
//! The casting operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::IndexingOperatorOperandParser;
use crate::syntax::OperatorExpression;
use crate::syntax::OperatorExpressionBuilder;
use crate::syntax::OperatorExpressionOperator;
use crate::Error;

#[derive(Default)]
pub struct Parser {
    builder: OperatorExpressionBuilder,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<OperatorExpression, Error> {
        let peek = stream.borrow_mut().peek();
        match peek {
            Some(Ok(Token {
                lexeme: Lexeme::Symbol(Symbol::ExclamationMark),
                location,
            })) => {
                stream.borrow_mut().next();
                self.builder.set_location(location);
                let rpn = Self::default().parse(stream.clone())?;
                self.builder.extend_with_expression(rpn);
                self.builder
                    .push_operator(location, OperatorExpressionOperator::Not);
                Ok(self.builder.finish())
            }
            Some(Ok(Token {
                lexeme: Lexeme::Symbol(Symbol::Minus),
                location,
            })) => {
                stream.borrow_mut().next();
                self.builder.set_location(location);
                let rpn = Self::default().parse(stream.clone())?;
                self.builder.extend_with_expression(rpn);
                self.builder
                    .push_operator(location, OperatorExpressionOperator::Negation);
                Ok(self.builder.finish())
            }
            Some(Ok(Token { location, .. })) => {
                self.builder.set_location(location);
                let rpn = IndexingOperatorOperandParser::default().parse(stream.clone())?;
                self.builder.extend_with_expression(rpn);
                Ok(self.builder.finish())
            }
            Some(Err(error)) => Err(Error::Lexical(error)),
            None => Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                stream.borrow().location(),
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::lexical;
    use crate::lexical::IntegerLiteral;
    use crate::lexical::Location;
    use crate::lexical::TokenStream;
    use crate::syntax::Literal;
    use crate::syntax::OperatorExpression;
    use crate::syntax::OperatorExpressionElement;
    use crate::syntax::OperatorExpressionObject;
    use crate::syntax::OperatorExpressionOperand;

    #[test]
    fn ok() {
        let code = r#"42 "#;

        let expected = OperatorExpression::new(
            Location::new(1, 1),
            vec![OperatorExpressionElement::new(
                Location::new(1, 1),
                OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                    Literal::new(
                        Location::new(1, 1),
                        lexical::Literal::Integer(IntegerLiteral::decimal("42".to_owned())),
                    ),
                )),
            )],
        );

        let result = Parser::default()
            .parse(Rc::new(RefCell::new(TokenStream::new(code.to_owned()))))
            .expect("Syntax error");

        assert_eq!(expected, result);
    }
}
