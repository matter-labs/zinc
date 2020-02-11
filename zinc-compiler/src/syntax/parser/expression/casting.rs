//!
//! The casting operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::AccessOperandParser;
use crate::syntax::Expression;
use crate::syntax::ExpressionBuilder;
use crate::syntax::ExpressionOperator;

#[derive(Default)]
pub struct Parser {
    builder: ExpressionBuilder,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(Expression, Option<Token>), Error> {
        match crate::syntax::take_or_next(initial.take(), stream.clone())? {
            Token {
                lexeme: Lexeme::Symbol(Symbol::ExclamationMark),
                location,
            } => {
                self.builder.set_location(location);
                let (expression, next) = Self::default().parse(stream, None)?;
                self.builder.extend_with_expression(expression);
                self.builder
                    .push_operator(location, ExpressionOperator::Not);
                Ok((self.builder.finish(), next))
            }
            Token {
                lexeme: Lexeme::Symbol(Symbol::Minus),
                location,
            } => {
                self.builder.set_location(location);
                let (expression, next) = Self::default().parse(stream, None)?;
                self.builder.extend_with_expression(expression);
                self.builder
                    .push_operator(location, ExpressionOperator::Negation);
                Ok((self.builder.finish(), next))
            }
            token => {
                self.builder.set_location(token.location);
                let (expression, next) =
                    AccessOperandParser::default().parse(stream, Some(token))?;
                self.builder.extend_with_expression(expression);
                Ok((self.builder.finish(), next))
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
    use crate::syntax::Expression;
    use crate::syntax::ExpressionAuxiliary;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::ExpressionOperator;
    use crate::syntax::Identifier;
    use crate::syntax::IntegerLiteral;

    #[test]
    fn ok() {
        let input = r#"array[42]"#;

        let expected = Ok((
            Expression::new(
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
                        ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 7),
                                lexical::IntegerLiteral::new_decimal("42".to_owned()),
                            ),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 6),
                        ExpressionObject::Operator(ExpressionOperator::Index),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 10),
                        ExpressionObject::Auxiliary(ExpressionAuxiliary::PlaceEnd),
                    ),
                ],
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 10))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(expected, result);
    }
}
