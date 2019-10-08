//!
//! The casting operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::AccessOperandParser;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Expression;
use crate::syntax::ExpressionBuilder;
use crate::syntax::ExpressionOperator;
use crate::Error;

#[derive(Default)]
pub struct Parser {
    builder: ExpressionBuilder,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<Expression, Error> {
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
                    .push_operator(location, ExpressionOperator::Not);
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
                    .push_operator(location, ExpressionOperator::Negation);
                Ok(self.builder.finish())
            }
            Some(Ok(Token { location, .. })) => {
                self.builder.set_location(location);
                let rpn = AccessOperandParser::default().parse(stream.clone())?;
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
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::ExpressionOperator;
    use crate::syntax::Identifier;
    use crate::syntax::Literal;

    #[test]
    fn ok() {
        let input = r#"array[42] "#;

        let expected = Ok(Expression::new(
            Location::new(1, 1),
            vec![
                ExpressionElement::new(
                    Location::new(1, 1),
                    ExpressionObject::Operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::new(1, 1),
                        "array".to_owned(),
                    ))),
                ),
                ExpressionElement::new(
                    Location::new(1, 7),
                    ExpressionObject::Operand(ExpressionOperand::Literal(Literal::new(
                        Location::new(1, 7),
                        lexical::Literal::Integer(IntegerLiteral::new_decimal("42".to_owned())),
                    ))),
                ),
                ExpressionElement::new(
                    Location::new(1, 6),
                    ExpressionObject::Operator(ExpressionOperator::Indexing),
                ),
            ],
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }
}
