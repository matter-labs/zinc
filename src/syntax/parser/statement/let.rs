//!
//! The let statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::ExpressionParser;
use crate::syntax::Identifier;
use crate::syntax::Let;
use crate::syntax::LetBuilder;
use crate::syntax::TypeParser;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Keyword,
    MutOrIdentifier,
    Identifier,
    ColonOrEquals,
    Type,
    Equals,
    Expression,
    End,
}

impl Default for State {
    fn default() -> Self {
        State::Keyword
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: LetBuilder,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<Let, Error> {
        loop {
            match self.state {
                State::Keyword => match stream.borrow_mut().next() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Keyword(Keyword::Let),
                        location,
                    })) => {
                        self.builder.set_location(location);
                        self.state = State::MutOrIdentifier;
                    }
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            ["let"].to_vec(),
                            lexeme,
                        )));
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::MutOrIdentifier => match stream.borrow_mut().next() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Keyword(Keyword::Mut),
                        ..
                    })) => {
                        self.builder.set_mutable();
                        self.state = State::Identifier;
                    }
                    Some(Ok(Token {
                        lexeme: Lexeme::Identifier(identifier),
                        location,
                    })) => {
                        let identifier = Identifier::new(location, identifier.name);
                        self.builder.set_identifier(identifier);
                        self.state = State::ColonOrEquals;
                    }
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            ["mut", "{identifier}"].to_vec(),
                            lexeme,
                        )));
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::Identifier => match stream.borrow_mut().next() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Identifier(identifier),
                        location,
                    })) => {
                        let identifier = Identifier::new(location, identifier.name);
                        self.builder.set_identifier(identifier);
                        self.state = State::ColonOrEquals;
                    }
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            ["{identifier}"].to_vec(),
                            lexeme,
                        )));
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::ColonOrEquals => match stream.borrow_mut().next() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::Colon),
                        ..
                    })) => self.state = State::Type,
                    Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::Equals),
                        ..
                    })) => self.state = State::Expression,
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            [":", "="].to_vec(),
                            lexeme,
                        )));
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::Type => {
                    let r#type = TypeParser::default().parse(stream.clone())?;
                    self.builder.set_type(r#type);
                    self.state = State::Equals;
                }
                State::Equals => match stream.borrow_mut().next() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::Equals),
                        ..
                    })) => self.state = State::Expression,
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            ["="].to_vec(),
                            lexeme,
                        )));
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::Expression => {
                    let expression = ExpressionParser::default().parse(stream.clone())?;
                    self.builder.set_expression(expression);
                    self.state = State::End;
                }
                State::End => return Ok(self.builder.finish()),
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
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::Identifier;
    use crate::syntax::Let;
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;

    #[test]
    fn ok() {
        let code = br#"let mut a: uint228 = 42;"#;

        let expected = Let::new(
            Location::new(1, 1),
            Identifier::new(Location::new(1, 9), b"a".to_vec()),
            Some(Type::new(Location::new(1, 12), TypeVariant::uint(228))),
            Expression::new(vec![ExpressionElement::new(
                ExpressionObject::Operand(ExpressionOperand::Literal(Literal::Integer(
                    IntegerLiteral::decimal(b"42".to_vec()),
                ))),
                Token::new(
                    Lexeme::Literal(Literal::Integer(IntegerLiteral::decimal(b"42".to_vec()))),
                    Location::new(1, 22),
                ),
            )]),
            true,
        );

        let result = Parser::default()
            .parse(Rc::new(RefCell::new(TokenStream::new(code.to_vec()))))
            .expect("Syntax error");

        assert_eq!(expected, result);
    }
}
