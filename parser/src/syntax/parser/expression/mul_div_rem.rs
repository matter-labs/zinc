//!
//! The multiplication/division/remainder operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Location;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::CastingOperandParser;
use crate::syntax::Expression;
use crate::syntax::ExpressionBuilder;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
use crate::syntax::TypeParser;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    CastingFirstOperand,
    CastingOperator,
    CastingSecondOperand,
}

impl Default for State {
    fn default() -> Self {
        State::CastingFirstOperand
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: ExpressionBuilder,
    operator: Option<(Location, ExpressionOperator)>,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<Expression, Error> {
        loop {
            match self.state {
                State::CastingFirstOperand => {
                    let rpn = CastingOperandParser::default().parse(stream.clone())?;
                    self.builder.set_location_if_unset(rpn.location);
                    self.builder.extend_with_expression(rpn);
                    if let Some((location, operator)) = self.operator.take() {
                        self.builder.push_operator(location, operator);
                    }
                    self.state = State::CastingOperator;
                }
                State::CastingOperator => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Keyword(Keyword::As),
                            location,
                        })) => {
                            stream.borrow_mut().next();
                            self.operator = Some((location, ExpressionOperator::Casting));
                            self.state = State::CastingSecondOperand;
                        }
                        _ => return Ok(self.builder.finish()),
                    }
                }
                State::CastingSecondOperand => {
                    let r#type = TypeParser::default().parse(stream.clone())?;
                    self.builder
                        .push_operand(r#type.location, ExpressionOperand::Type(r#type));
                    if let Some((location, operator)) = self.operator.take() {
                        self.builder.push_operator(location, operator);
                    }
                    self.state = State::CastingOperator;
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
    use crate::lexical::IntegerLiteral;
    use crate::lexical::Location;
    use crate::lexical::TokenStream;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::ExpressionOperator;
    use crate::syntax::Literal;
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;

    #[test]
    fn ok() {
        let input = r#"42 as field "#;

        let expected = Ok(Expression::new(
            Location::new(1, 1),
            vec![
                ExpressionElement::new(
                    Location::new(1, 1),
                    ExpressionObject::Operand(ExpressionOperand::Literal(Literal::new(
                        Location::new(1, 1),
                        lexical::Literal::Integer(IntegerLiteral::new_decimal("42".to_owned())),
                    ))),
                ),
                ExpressionElement::new(
                    Location::new(1, 7),
                    ExpressionObject::Operand(ExpressionOperand::Type(Type::new(
                        Location::new(1, 7),
                        TypeVariant::new_field(),
                    ))),
                ),
                ExpressionElement::new(
                    Location::new(1, 4),
                    ExpressionObject::Operator(ExpressionOperator::Casting),
                ),
            ],
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }
}
