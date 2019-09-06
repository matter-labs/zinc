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
use crate::syntax::LetStatement;
use crate::syntax::LetStatementBuilder;
use crate::syntax::TypeParser;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordLet,
    MutOrIdentifier,
    Identifier,
    ColonOrEquals,
    Type,
    Equals,
    Expression,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordLet
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: LetStatementBuilder,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<LetStatement, Error> {
        loop {
            match self.state {
                State::KeywordLet => match stream.borrow_mut().next() {
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
                        let identifier = Identifier::new(location, identifier.name().to_owned());
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
                        let identifier = Identifier::new(location, identifier.name().to_owned());
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
                    return Ok(self.builder.finish());
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
    use crate::syntax::LetStatement;
    use crate::syntax::OperatorExpression;
    use crate::syntax::OperatorExpressionElement;
    use crate::syntax::OperatorExpressionObject;
    use crate::syntax::OperatorExpressionOperand;
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;

    #[test]
    fn ok() {
        let code = r#"let mut a: uint232 = 42 "#;

        let expected = LetStatement::new(
            Location::new(1, 1),
            Identifier::new(Location::new(1, 9), "a".to_owned()),
            true,
            Some(Type::new(Location::new(1, 12), TypeVariant::uint(232))),
            Expression::Operator(OperatorExpression::new(vec![
                OperatorExpressionElement::new(
                    OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                        Literal::Integer(IntegerLiteral::decimal("42".to_owned())),
                    )),
                    Token::new(
                        Lexeme::Literal(Literal::Integer(IntegerLiteral::decimal("42".to_owned()))),
                        Location::new(1, 22),
                    ),
                ),
            ])),
        );

        let result = Parser::default()
            .parse(Rc::new(RefCell::new(TokenStream::new(code.to_owned()))))
            .expect("Syntax error");

        assert_eq!(expected, result);
    }
}
