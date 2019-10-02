//!
//! The block expression parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Literal;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::ArrayExpression;
use crate::syntax::ArrayExpressionBuilder;
use crate::syntax::Error as SyntaxError;
use crate::syntax::ExpressionParser;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    BracketSquareOpen,
    FirstExpressionOrBracketSquareClose,
    ExpressionOrBracketSquareClose,
    CommaOrBracketSquareClose,
    CommaOrSemicolonOrBracketSquareClose,
    SizeLiteral,
    BracketSquareClose,
}

impl Default for State {
    fn default() -> Self {
        State::BracketSquareOpen
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: ArrayExpressionBuilder,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<ArrayExpression, Error> {
        loop {
            match self.state {
                State::BracketSquareOpen => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareLeft),
                            location,
                        })) => {
                            self.builder.set_location(location);
                            self.state = State::FirstExpressionOrBracketSquareClose;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["["],
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
                State::FirstExpressionOrBracketSquareClose => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        })) => {
                            stream.borrow_mut().next();
                            return Ok(self.builder.finish());
                        }
                        Some(Ok(..)) => {
                            let expression = ExpressionParser::default().parse(stream.clone())?;
                            self.builder.push_expression(expression);
                            self.state = State::CommaOrSemicolonOrBracketSquareClose;
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
                State::ExpressionOrBracketSquareClose => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        })) => {
                            stream.borrow_mut().next();
                            return Ok(self.builder.finish());
                        }
                        Some(Ok(..)) => {
                            let expression = ExpressionParser::default().parse(stream.clone())?;
                            self.builder.push_expression(expression);
                            self.state = State::CommaOrBracketSquareClose;
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
                State::CommaOrBracketSquareClose => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Comma),
                            ..
                        })) => {
                            self.state = State::ExpressionOrBracketSquareClose;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        })) => return Ok(self.builder.finish()),
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![",", "]"],
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
                State::CommaOrSemicolonOrBracketSquareClose => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Comma),
                            ..
                        })) => {
                            self.state = State::ExpressionOrBracketSquareClose;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Semicolon),
                            ..
                        })) => {
                            self.state = State::SizeLiteral;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        })) => return Ok(self.builder.finish()),
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![",", ";", "]"],
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
                State::SizeLiteral => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Literal(Literal::Integer(integer)),
                            ..
                        })) => {
                            self.builder.fill(integer);
                            self.state = State::BracketSquareClose;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{integer}"],
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
                State::BracketSquareClose => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        })) => return Ok(self.builder.finish()),
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["]"],
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
    use crate::syntax::ArrayExpression;
    use crate::syntax::Expression;
    use crate::syntax::Literal;
    use crate::syntax::OperatorExpression;
    use crate::syntax::OperatorExpressionElement;
    use crate::syntax::OperatorExpressionObject;
    use crate::syntax::OperatorExpressionOperand;

    #[test]
    fn ok() {
        let code = r#"[1, 2, 3]"#;

        let expected = Ok(ArrayExpression::new(
            Location::new(1, 1),
            vec![
                Expression::Operator(OperatorExpression::new(
                    Location::new(1, 2),
                    vec![OperatorExpressionElement::new(
                        Location::new(1, 2),
                        OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                            Literal::new(
                                Location::new(1, 2),
                                lexical::Literal::Integer(IntegerLiteral::decimal("1".to_owned())),
                            ),
                        )),
                    )],
                )),
                Expression::Operator(OperatorExpression::new(
                    Location::new(1, 5),
                    vec![OperatorExpressionElement::new(
                        Location::new(1, 5),
                        OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                            Literal::new(
                                Location::new(1, 5),
                                lexical::Literal::Integer(IntegerLiteral::decimal("2".to_owned())),
                            ),
                        )),
                    )],
                )),
                Expression::Operator(OperatorExpression::new(
                    Location::new(1, 8),
                    vec![OperatorExpressionElement::new(
                        Location::new(1, 8),
                        OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                            Literal::new(
                                Location::new(1, 8),
                                lexical::Literal::Integer(IntegerLiteral::decimal("3".to_owned())),
                            ),
                        )),
                    )],
                )),
            ],
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(code.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_empty() {
        let code = r#"[]"#;

        let expected = Ok(ArrayExpression::new(Location::new(1, 1), vec![]));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(code.to_owned()))));

        assert_eq!(expected, result);
    }
}
