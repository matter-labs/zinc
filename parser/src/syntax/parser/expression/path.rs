//!
//! The path expression parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Location;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Expression;
use crate::syntax::ExpressionBuilder;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
use crate::syntax::Identifier;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Identifier,
    DoubleColonOrEnd,
}

impl Default for State {
    fn default() -> Self {
        State::Identifier
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
                State::Identifier => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        })) => {
                            self.builder.set_location_if_unset(location);
                            self.builder.push_operand(
                                location,
                                ExpressionOperand::Identifier(Identifier::new(
                                    location,
                                    identifier.name,
                                )),
                            );
                            if let Some((location, operator)) = self.operator.take() {
                                self.builder.push_operator(location, operator);
                            }
                            self.state = State::DoubleColonOrEnd;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{identifier}"],
                                lexeme,
                            )));
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
                State::DoubleColonOrEnd => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleColon),
                            location,
                        })) => {
                            stream.borrow_mut().next();
                            self.operator = Some((location, ExpressionOperator::Path));
                            self.state = State::Identifier;
                        }
                        _ => return Ok(self.builder.finish()),
                    }
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
    use crate::lexical::Location;
    use crate::lexical::TokenStream;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::ExpressionOperator;
    use crate::syntax::Identifier;

    #[test]
    fn ok() {
        let input = r#"mega::ultra::namespace;"#;

        let expected = Ok(Expression::new(
            Location::new(1, 1),
            vec![
                ExpressionElement::new(
                    Location::new(1, 1),
                    ExpressionObject::Operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::new(1, 1),
                        "Test".to_owned(),
                    ))),
                ),
                ExpressionElement::new(
                    Location::new(1, 7),
                    ExpressionObject::Operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::new(1, 7),
                        "Test".to_owned(),
                    ))),
                ),
                ExpressionElement::new(
                    Location::new(1, 5),
                    ExpressionObject::Operator(ExpressionOperator::Path),
                ),
                ExpressionElement::new(
                    Location::new(1, 14),
                    ExpressionObject::Operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::new(1, 14),
                        "Test".to_owned(),
                    ))),
                ),
                ExpressionElement::new(
                    Location::new(1, 12),
                    ExpressionObject::Operator(ExpressionOperator::Path),
                ),
            ],
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));;

        assert_eq!(expected, result);
    }
}
