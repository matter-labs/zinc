//!
//! The block expression parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::BlockExpression;
use crate::syntax::BlockExpressionBuilder;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Statement;
use crate::syntax::StatementParser;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    BracketCurlyOpen,
    StatementOrBracketCurlyClose,
    BracketCurlyClose,
}

impl Default for State {
    fn default() -> Self {
        State::BracketCurlyOpen
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: BlockExpressionBuilder,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<BlockExpression, Error> {
        loop {
            match self.state {
                State::BracketCurlyOpen => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            location,
                        })) => {
                            self.builder.set_location(location);
                            self.state = State::StatementOrBracketCurlyClose;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                ["{"].to_vec(),
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
                State::StatementOrBracketCurlyClose => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        })) => {
                            stream.borrow_mut().next();
                            return Ok(self.builder.finish());
                        }
                        Some(Ok(..)) => {
                            let (statement, is_unterminated) =
                                StatementParser::default().parse(stream.clone())?;
                            match statement {
                                Statement::Expression(expression) => {
                                    if is_unterminated {
                                        self.builder.set_expression(expression);
                                        self.state = State::BracketCurlyClose;
                                    } else {
                                        self.builder
                                            .push_statement(Statement::Expression(expression));
                                    }
                                }
                                statement => self.builder.push_statement(statement),
                            }
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
                State::BracketCurlyClose => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        })) => return Ok(self.builder.finish()),
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                ["}"].to_vec(),
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
    use crate::syntax::BlockExpression;
    use crate::syntax::DebugStatement;
    use crate::syntax::Expression;
    use crate::syntax::Literal;
    use crate::syntax::OperatorExpression;
    use crate::syntax::OperatorExpressionElement;
    use crate::syntax::OperatorExpressionObject;
    use crate::syntax::OperatorExpressionOperand;
    use crate::syntax::OperatorExpressionOperator;
    use crate::syntax::Statement;

    #[test]
    fn ok() {
        let code = r#"{ debug(42); 2 + 1 }"#;

        let expected = BlockExpression::new(
            Location::new(1, 1),
            vec![Statement::Debug(DebugStatement::new(
                Location::new(1, 3),
                Expression::Operator(OperatorExpression::new(
                    Location::new(1, 9),
                    vec![OperatorExpressionElement::new(
                        Location::new(1, 9),
                        OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                            Literal::new(
                                Location::new(1, 9),
                                lexical::Literal::Integer(IntegerLiteral::decimal("42".to_owned())),
                            ),
                        )),
                    )],
                )),
            ))],
            Some(Expression::Operator(OperatorExpression::new(
                Location::new(1, 14),
                vec![
                    OperatorExpressionElement::new(
                        Location::new(1, 14),
                        OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                            Literal::new(
                                Location::new(1, 14),
                                lexical::Literal::Integer(IntegerLiteral::decimal("2".to_owned())),
                            ),
                        )),
                    ),
                    OperatorExpressionElement::new(
                        Location::new(1, 18),
                        OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                            Literal::new(
                                Location::new(1, 18),
                                lexical::Literal::Integer(IntegerLiteral::decimal("1".to_owned())),
                            ),
                        )),
                    ),
                    OperatorExpressionElement::new(
                        Location::new(1, 16),
                        OperatorExpressionObject::Operator(OperatorExpressionOperator::Addition),
                    ),
                ],
            ))),
        );

        let result = Parser::default()
            .parse(Rc::new(RefCell::new(TokenStream::new(code.to_owned()))))
            .expect("Syntax error");

        assert_eq!(expected, result);
    }
}
