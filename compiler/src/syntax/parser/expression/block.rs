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
    BracketCurlyLeft,
    StatementOrBracketCurlyRight,
    BracketCurlyRight,
}

impl Default for State {
    fn default() -> Self {
        State::BracketCurlyLeft
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
                State::BracketCurlyLeft => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            location,
                        })) => {
                            self.builder.set_location(location);
                            self.state = State::StatementOrBracketCurlyRight;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{"],
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
                State::StatementOrBracketCurlyRight => {
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
                                        self.state = State::BracketCurlyRight;
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
                State::BracketCurlyRight => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        })) => return Ok(self.builder.finish()),
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["}"],
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
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::ExpressionOperator;
    use crate::syntax::Literal;
    use crate::syntax::Statement;

    #[test]
    fn ok_statements_with_expression() {
        let input = r#"{ debug(42); 2 + 1 }"#;

        let expected = Ok(BlockExpression::new(
            Location::new(1, 1),
            vec![Statement::Debug(DebugStatement::new(
                Location::new(1, 3),
                Expression::new(
                    Location::new(1, 9),
                    vec![ExpressionElement::new(
                        Location::new(1, 9),
                        ExpressionObject::Operand(ExpressionOperand::Literal(Literal::new(
                            Location::new(1, 9),
                            lexical::Literal::Integer(IntegerLiteral::new_decimal("42".to_owned())),
                        ))),
                    )],
                ),
            ))],
            Some(Expression::new(
                Location::new(1, 14),
                vec![
                    ExpressionElement::new(
                        Location::new(1, 14),
                        ExpressionObject::Operand(ExpressionOperand::Literal(Literal::new(
                            Location::new(1, 14),
                            lexical::Literal::Integer(IntegerLiteral::new_decimal("2".to_owned())),
                        ))),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 18),
                        ExpressionObject::Operand(ExpressionOperand::Literal(Literal::new(
                            Location::new(1, 18),
                            lexical::Literal::Integer(IntegerLiteral::new_decimal("1".to_owned())),
                        ))),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 16),
                        ExpressionObject::Operator(ExpressionOperator::Addition),
                    ),
                ],
            )),
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_empty() {
        let input = r#"{}"#;

        let expected = Ok(BlockExpression::new(Location::new(1, 1), vec![], None));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }
}
