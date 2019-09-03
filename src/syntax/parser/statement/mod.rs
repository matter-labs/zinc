//!
//! The statement parser.
//!

mod debug;
mod r#let;
mod require;

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Expression;
use crate::syntax::ExpressionParser;
use crate::syntax::Statement;
use crate::Error;

use self::debug::Parser as DebugParser;
use self::r#let::Parser as LetParser;
use self::require::Parser as RequireParser;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Statement,
    Semicolon,
    SemicolonOrEnd,
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
                        Some(Ok(..)) => {
                            let result = ExpressionParser::default().parse(stream.clone())?;
                            match result {
                                Expression::Operator(..) => {
                                    self.state = State::SemicolonOrEnd;
                                    Statement::Expression(result)
                                }
                                Expression::Block(..) => {
                                    return Ok((Statement::Expression(result), false))
                                }
                            }
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                    });
                }
                State::Semicolon => match stream.borrow_mut().next() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::Semicolon),
                        ..
                    })) => return Ok((self.statement.take().expect("Option state bug"), false)),
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            [";"].to_vec(),
                            lexeme,
                        )));
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::SemicolonOrEnd => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Semicolon),
                            ..
                        })) => {
                            stream.borrow_mut().next();
                            return Ok((self.statement.take().expect("Option state bug"), false));
                        }
                        Some(Ok(..)) => {
                            return Ok((self.statement.take().expect("Option state bug"), true));
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
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
    use crate::lexical::IntegerLiteral;
    use crate::lexical::Lexeme;
    use crate::lexical::Literal;
    use crate::lexical::Location;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::Expression;
    use crate::syntax::Identifier;
    use crate::syntax::Let;
    use crate::syntax::OperatorExpression;
    use crate::syntax::OperatorExpressionElement;
    use crate::syntax::OperatorExpressionObject;
    use crate::syntax::OperatorExpressionOperand;
    use crate::syntax::Statement;
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;

    #[test]
    fn ok() {
        let code = br#"let mut a: uint228 = 42;"#;

        let expected = Statement::Let(Let::new(
            Location::new(1, 1),
            Identifier::new(Location::new(1, 9), b"a".to_vec()),
            true,
            Some(Type::new(Location::new(1, 12), TypeVariant::uint(228))),
            Expression::Operator(OperatorExpression::new(vec![
                OperatorExpressionElement::new(
                    OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                        Literal::Integer(IntegerLiteral::decimal(b"42".to_vec())),
                    )),
                    Token::new(
                        Lexeme::Literal(Literal::Integer(IntegerLiteral::decimal(b"42".to_vec()))),
                        Location::new(1, 22),
                    ),
                ),
            ])),
        ));

        let (result, _is_unterminated) = Parser::default()
            .parse(Rc::new(RefCell::new(TokenStream::new(code.to_vec()))))
            .expect("Syntax error");

        assert_eq!(expected, result);
    }
}
