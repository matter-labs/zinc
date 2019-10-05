//!
//! The debug statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::DebugStatement;
use crate::syntax::DebugStatementBuilder;
use crate::syntax::Error as SyntaxError;
use crate::syntax::ExpressionParser;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordDebug,
    ParenthesisLeft,
    Expression,
    ParenthesisRight,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordDebug
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: DebugStatementBuilder,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<DebugStatement, Error> {
        loop {
            match self.state {
                State::KeywordDebug => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Keyword(Keyword::Debug),
                            location,
                        })) => {
                            self.builder.set_location(location);
                            self.state = State::ParenthesisLeft;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["debug"],
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
                State::ParenthesisLeft => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                            ..
                        })) => self.state = State::Expression,
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["("],
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
                State::Expression => {
                    let expression = ExpressionParser::default().parse(stream.clone())?;
                    self.builder.set_expression(expression);
                    self.state = State::ParenthesisRight;
                }
                State::ParenthesisRight => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        })) => return Ok(self.builder.finish()),
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![")"],
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
    use crate::syntax::DebugStatement;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::Literal;

    #[test]
    fn ok() {
        let input = r#"debug(42);"#;

        let expected = Ok(DebugStatement::new(
            Location::new(1, 1),
            Expression::new(
                Location::new(1, 7),
                vec![ExpressionElement::new(
                    Location::new(1, 7),
                    ExpressionObject::Operand(ExpressionOperand::Literal(Literal::new(
                        Location::new(1, 7),
                        lexical::Literal::Integer(IntegerLiteral::new_decimal("42".to_owned())),
                    ))),
                )],
            ),
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }
}
