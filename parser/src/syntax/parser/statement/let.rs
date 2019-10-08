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
                State::KeywordLet => {
                    let next = stream.borrow_mut().next();
                    match next {
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
                                vec!["let"],
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
                State::MutOrIdentifier => {
                    let next = stream.borrow_mut().next();
                    match next {
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
                                vec!["mut", "{identifier}"],
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
                State::Identifier => {
                    let next = stream.borrow_mut().next();
                    match next {
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
                State::ColonOrEquals => {
                    let next = stream.borrow_mut().next();
                    match next {
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
                                vec![":", "="],
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
                State::Type => {
                    let r#type = TypeParser::default().parse(stream.clone())?;
                    self.builder.set_type(r#type);
                    self.state = State::Equals;
                }
                State::Equals => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Equals),
                            ..
                        })) => self.state = State::Expression,
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["="],
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
    use crate::lexical;
    use crate::lexical::IntegerLiteral;
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Symbol;
    use crate::lexical::TokenStream;
    use crate::syntax::Error as SyntaxError;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::Identifier;
    use crate::syntax::LetStatement;
    use crate::syntax::Literal;
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;
    use crate::Error;

    #[test]
    fn ok_simple() {
        let input = r#"let a = 42;"#;

        let expected = Ok(LetStatement::new(
            Location::new(1, 1),
            Identifier::new(Location::new(1, 5), "a".to_owned()),
            false,
            None,
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
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_mut_with_type() {
        let input = r#"let mut a: u232 = 42;"#;

        let expected = Ok(LetStatement::new(
            Location::new(1, 1),
            Identifier::new(Location::new(1, 9), "a".to_owned()),
            true,
            Some(Type::new(
                Location::new(1, 12),
                TypeVariant::new_integer_unsigned(232),
            )),
            Expression::new(
                Location::new(1, 19),
                vec![ExpressionElement::new(
                    Location::new(1, 19),
                    ExpressionObject::Operand(ExpressionOperand::Literal(Literal::new(
                        Location::new(1, 19),
                        lexical::Literal::Integer(IntegerLiteral::new_decimal("42".to_owned())),
                    ))),
                )],
            ),
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn err_no_value() {
        let input = r#"let a;"#;

        let expected = Err(Error::Syntax(SyntaxError::Expected(
            Location::new(1, 6),
            vec![":", "="],
            Lexeme::Symbol(Symbol::Semicolon),
        )));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }
}
