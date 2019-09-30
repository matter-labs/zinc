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
use crate::syntax::CastingOperatorOperandParser;
use crate::syntax::OperatorExpression;
use crate::syntax::OperatorExpressionBuilder;
use crate::syntax::OperatorExpressionOperand;
use crate::syntax::OperatorExpressionOperator;
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
    builder: OperatorExpressionBuilder,
    operator: Option<(Location, OperatorExpressionOperator)>,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<OperatorExpression, Error> {
        loop {
            match self.state {
                State::CastingFirstOperand => {
                    let rpn = CastingOperatorOperandParser::default().parse(stream.clone())?;
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
                            self.operator = Some((location, OperatorExpressionOperator::Casting));
                            self.state = State::CastingSecondOperand;
                        }
                        _ => return Ok(self.builder.finish()),
                    }
                }
                State::CastingSecondOperand => {
                    let r#type = TypeParser::default().parse(stream.clone())?;
                    self.builder
                        .push_operand(r#type.location, OperatorExpressionOperand::Type(r#type));
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
    use crate::syntax::Literal;
    use crate::syntax::OperatorExpression;
    use crate::syntax::OperatorExpressionElement;
    use crate::syntax::OperatorExpressionObject;
    use crate::syntax::OperatorExpressionOperand;
    use crate::syntax::OperatorExpressionOperator;
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;

    #[test]
    fn ok() {
        let code = r#"42 as field "#;

        let expected = OperatorExpression::new(
            Location::new(1, 1),
            vec![
                OperatorExpressionElement::new(
                    Location::new(1, 1),
                    OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                        Literal::new(
                            Location::new(1, 1),
                            lexical::Literal::Integer(IntegerLiteral::decimal("42".to_owned())),
                        ),
                    )),
                ),
                OperatorExpressionElement::new(
                    Location::new(1, 7),
                    OperatorExpressionObject::Operand(OperatorExpressionOperand::Type(Type::new(
                        Location::new(1, 7),
                        TypeVariant::Field,
                    ))),
                ),
                OperatorExpressionElement::new(
                    Location::new(1, 4),
                    OperatorExpressionObject::Operator(OperatorExpressionOperator::Casting),
                ),
            ],
        );

        let result = Parser::default()
            .parse(Rc::new(RefCell::new(TokenStream::new(code.to_owned()))))
            .expect("Syntax error");

        assert_eq!(expected, result);
    }
}
