//!
//! The statement parser.
//!

mod debug;
mod r#let;
mod r#loop;
mod require;

pub use self::debug::Parser as DebugParser;
pub use self::r#let::Parser as LetParser;
pub use self::r#loop::Parser as LoopParser;
pub use self::require::Parser as RequireParser;

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::ExpressionParser;
use crate::syntax::Statement;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Statement,
    Semicolon,
    SemicolonOptional,
}

impl Default for State {
    fn default() -> Self {
        State::Statement
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    statement: Option<Statement>,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<(Statement, bool), Error> {
        loop {
            match self.state {
                State::Statement => {
                    let peek = stream.borrow_mut().peek();
                    self.statement = Some(match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Keyword(Keyword::Let),
                            ..
                        })) => {
                            let result = LetParser::default()
                                .parse(stream.clone())
                                .map(Statement::Let)?;
                            self.state = State::Semicolon;
                            result
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Keyword(Keyword::Require),
                            ..
                        })) => {
                            let result = RequireParser::default()
                                .parse(stream.clone())
                                .map(Statement::Require)?;
                            self.state = State::Semicolon;
                            result
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Keyword(Keyword::Debug),
                            ..
                        })) => {
                            let result = DebugParser::default()
                                .parse(stream.clone())
                                .map(Statement::Debug)?;
                            self.state = State::Semicolon;
                            result
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Keyword(Keyword::For),
                            ..
                        })) => {
                            let result = LoopParser::default()
                                .parse(stream.clone())
                                .map(Statement::Loop)?;
                            self.state = State::Semicolon;
                            result
                        }
                        Some(Ok(..)) => {
                            let result = ExpressionParser::default().parse(stream.clone())?;
                            self.state = State::SemicolonOptional;
                            Statement::Expression(result)
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    });
                }
                State::Semicolon => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Semicolon),
                            ..
                        })) => {
                            return Ok((
                                self.statement.take().expect("Always contains a value"),
                                false,
                            ))
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![";"],
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
                State::SemicolonOptional => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Semicolon),
                            ..
                        })) => {
                            stream.borrow_mut().next();
                            return Ok((
                                self.statement.take().expect("Always contains a value"),
                                false,
                            ));
                        }
                        Some(Ok(..)) => {
                            return Ok((
                                self.statement.take().expect("Always contains a value"),
                                true,
                            ));
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Ok((
                                self.statement.take().expect("Always contains a value"),
                                true,
                            ))
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
    use crate::syntax::Expression;
    use crate::syntax::Identifier;
    use crate::syntax::LetStatement;
    use crate::syntax::Literal;
    use crate::syntax::OperatorExpression;
    use crate::syntax::OperatorExpressionElement;
    use crate::syntax::OperatorExpressionObject;
    use crate::syntax::OperatorExpressionOperand;
    use crate::syntax::Statement;
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;

    #[test]
    fn ok_semicolon_terminated() {
        let code = r#"let mut a: uint232 = 42;"#;

        let expected = Ok((
            Statement::Let(LetStatement::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 9), "a".to_owned()),
                true,
                Some(Type::new(Location::new(1, 12), TypeVariant::uint(232))),
                Expression::Operator(OperatorExpression::new(
                    Location::new(1, 22),
                    vec![OperatorExpressionElement::new(
                        Location::new(1, 22),
                        OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                            Literal::new(
                                Location::new(1, 22),
                                lexical::Literal::Integer(IntegerLiteral::decimal("42".to_owned())),
                            ),
                        )),
                    )],
                )),
            )),
            false,
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(code.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_semicolon_unterminated() {
        let code = r#"{ 42 } "#;

        let expected = Ok((
            Statement::Expression(Expression::Block(BlockExpression {
                location: Location::new(1, 1),
                statements: vec![],
                expression: Some(Box::new(Expression::Operator(OperatorExpression::new(
                    Location::new(1, 3),
                    vec![OperatorExpressionElement::new(
                        Location::new(1, 3),
                        OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                            Literal::new(
                                Location::new(1, 3),
                                lexical::Literal::Integer(IntegerLiteral::decimal("42".to_owned())),
                            ),
                        )),
                    )],
                )))),
            })),
            true,
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(code.to_owned()))));

        assert_eq!(expected, result);
    }
}
