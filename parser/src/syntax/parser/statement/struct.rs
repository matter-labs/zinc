//!
//! The struct statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Identifier;
use crate::syntax::StructStatement;
use crate::syntax::StructStatementBuilder;
use crate::syntax::TypeParser;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordStruct,
    Identifier,
    BracketCurlyLeftOrEnd,
    IdentifierOrBracketCurlyRight,
    Colon,
    Type,
    CommaOrBracketCurlyRight,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordStruct
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: StructStatementBuilder,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<StructStatement, Error> {
        loop {
            match self.state {
                State::KeywordStruct => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Keyword(Keyword::Struct),
                            location,
                        })) => {
                            self.builder.set_location(location);
                            self.state = State::Identifier;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["struct"],
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
                            self.state = State::BracketCurlyLeftOrEnd;
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
                State::BracketCurlyLeftOrEnd => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            ..
                        })) => {
                            stream.borrow_mut().next();
                            self.state = State::IdentifierOrBracketCurlyRight;
                        }
                        Some(Ok(..)) => return Ok(self.builder.finish()),
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
                State::IdentifierOrBracketCurlyRight => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        })) => return Ok(self.builder.finish()),
                        Some(Ok(Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        })) => {
                            let identifier = Identifier::new(location, identifier.name);
                            self.builder.push_field_identifier(identifier);
                            self.state = State::Colon;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{identifier}", "}"],
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
                State::Colon => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Colon),
                            ..
                        })) => self.state = State::Type,
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![":"],
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
                    self.builder.set_field_type(r#type);
                    self.state = State::CommaOrBracketCurlyRight;
                }
                State::CommaOrBracketCurlyRight => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Comma),
                            ..
                        })) => self.state = State::IdentifierOrBracketCurlyRight,
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        })) => return Ok(self.builder.finish()),
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![",", "}"],
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
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Symbol;
    use crate::lexical::TokenStream;
    use crate::syntax::Error as SyntaxError;
    use crate::syntax::Identifier;
    use crate::syntax::StructStatement;
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;
    use crate::Error;

    #[test]
    fn ok_single() {
        let input = r#"
    struct Test {
        a: u232,
    }
"#;

        let expected = Ok(StructStatement::new(
            Location::new(2, 5),
            Identifier::new(Location::new(2, 12), "Test".to_owned()),
            vec![(
                Identifier::new(Location::new(3, 9), "a".to_owned()),
                Type::new(Location::new(3, 12), TypeVariant::new_integer_unsigned(232)),
            )],
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_empty_with_brackets() {
        let input = r#"
    struct Test {}
"#;

        let expected = Ok(StructStatement::new(
            Location::new(2, 5),
            Identifier::new(Location::new(2, 12), "Test".to_owned()),
            vec![],
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));;

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_empty_with_semicolon() {
        let input = r#"
    struct Test;
"#;

        let expected = Ok(StructStatement::new(
            Location::new(2, 5),
            Identifier::new(Location::new(2, 12), "Test".to_owned()),
            vec![],
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));;

        assert_eq!(expected, result);
    }

    #[test]
    fn err_expected_comma() {
        let input = r#"
    struct Test {
        a: u232;
    }
"#;

        let expected = Err(Error::Syntax(SyntaxError::Expected(
            Location::new(3, 16),
            vec![",", "}"],
            Lexeme::Symbol(Symbol::Semicolon),
        )));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }
}
