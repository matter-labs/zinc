//!
//! The type parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Literal;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Type;
use crate::syntax::TypeBuilder;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordOrTupleParenthesis,
    TupleParenthesisRight,
    ArrayType,
    ArraySemicolon,
    ArraySize,
    ArrayBracketSquareRight,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordOrTupleParenthesis
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: TypeBuilder,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<Type, Error> {
        loop {
            match self.state {
                State::KeywordOrTupleParenthesis => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Keyword(keyword),
                            location,
                        })) => match keyword {
                            keyword @ Keyword::Bool
                            | keyword @ Keyword::Int { .. }
                            | keyword @ Keyword::Uint { .. }
                            | keyword @ Keyword::Field => {
                                self.builder.set_location(location);
                                self.builder.set_keyword(keyword);
                                return Ok(self.builder.finish());
                            }
                            _ => {
                                return Err(Error::Syntax(SyntaxError::Expected(
                                    location,
                                    vec!["{type}"],
                                    Lexeme::Keyword(keyword),
                                )))
                            }
                        },
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                            location,
                        })) => {
                            self.builder.set_location(location);
                            self.state = State::TupleParenthesisRight;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareLeft),
                            location,
                        })) => {
                            self.builder.set_location(location);
                            self.state = State::ArrayType;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{type}", "(", "["],
                                lexeme,
                            )))
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
                State::TupleParenthesisRight => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        })) => {
                            self.builder.set_void();
                            return Ok(self.builder.finish());
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![")"],
                                lexeme,
                            )))
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
                State::ArrayType => {
                    let array_type = Self::default().parse(stream.clone())?;
                    self.builder.set_array_type_variant(array_type.variant);
                    self.state = State::ArraySemicolon;
                }
                State::ArraySemicolon => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Semicolon),
                            ..
                        })) => {
                            self.state = State::ArraySize;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![";"],
                                lexeme,
                            )))
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
                State::ArraySize => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Literal(Literal::Integer(integer)),
                            ..
                        })) => {
                            self.builder.set_array_size(integer);
                            self.state = State::ArrayBracketSquareRight;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{integer}"],
                                lexeme,
                            )))
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
                State::ArrayBracketSquareRight => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        })) => {
                            return Ok(self.builder.finish());
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["]"],
                                lexeme,
                            )))
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
    use crate::lexical::Identifier;
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Symbol;
    use crate::lexical::TokenStream;
    use crate::syntax::Error as SyntaxError;
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;
    use crate::Error;

    #[test]
    fn ok_void() {
        let code = "()";

        let expected = Ok(Type::new(Location::new(1, 1), TypeVariant::Void));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(code.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_integer() {
        let code = "uint232";

        let expected = Ok(Type::new(Location::new(1, 1), TypeVariant::uint(232)));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(code.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_field() {
        let code = "field";

        let expected = Ok(Type::new(Location::new(1, 1), TypeVariant::Field));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(code.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_array() {
        let code = "[field; 8]";

        let expected = Ok(Type::new(
            Location::new(1, 1),
            TypeVariant::array(TypeVariant::Field, 8),
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(code.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_array_double() {
        let code = "[[field; 8]; 8]";

        let expected = Ok(Type::new(
            Location::new(1, 1),
            TypeVariant::array(TypeVariant::array(TypeVariant::Field, 8), 8),
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(code.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn err_integer_not_keyword() {
        let code = "uint19";

        let expected = Err(Error::Syntax(SyntaxError::Expected(
            Location::new(1, 1),
            vec!["{type}", "(", "["],
            Lexeme::Identifier(Identifier::new("uint19".to_owned())),
        )));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(code.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn err_array_expected_semicolon() {
        let code = "[field, 8]";

        let expected = Err(Error::Syntax(SyntaxError::Expected(
            Location::new(1, 7),
            vec![";"],
            Lexeme::Symbol(Symbol::Comma),
        )));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(code.to_owned()))));

        assert_eq!(expected, result);
    }
}
