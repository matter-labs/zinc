//!
//! The witnesses parser.
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
use crate::syntax::TypeParser;
use crate::syntax::Witness;
use crate::syntax::WitnessBuilder;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordWitness,
    BracketCurlyLeft,
    IdentifierOrBracketCurlyRight,
    Colon,
    Type,
    Comma,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordWitness
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    witnesses: Vec<Witness>,
    builder: WitnessBuilder,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<Vec<Witness>, Error> {
        let peek = stream.borrow_mut().peek();
        match peek {
            Some(Ok(Token {
                lexeme: Lexeme::Keyword(Keyword::Witness),
                ..
            })) => {}
            _ => return Ok(self.witnesses),
        }

        loop {
            match self.state {
                State::KeywordWitness => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Keyword(Keyword::Witness),
                            ..
                        })) => self.state = State::BracketCurlyLeft,
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["witness"],
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
                State::BracketCurlyLeft => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            ..
                        })) => self.state = State::IdentifierOrBracketCurlyRight,
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
                State::IdentifierOrBracketCurlyRight => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        })) => return Ok(self.witnesses),
                        Some(Ok(Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        })) => {
                            let identifier = Identifier::new(location, identifier.name);
                            self.builder.set_location(location);
                            self.builder.set_identifier(identifier);
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
                    self.builder.set_type(r#type);
                    self.state = State::Comma;
                }
                State::Comma => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Comma),
                            ..
                        })) => {
                            let witness = self.builder.build();
                            log::trace!("Witness: {:?}", witness);
                            self.witnesses.push(witness);
                            self.state = State::IdentifierOrBracketCurlyRight;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![","],
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
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;
    use crate::syntax::Witness;
    use crate::Error;

    #[test]
    fn ok_single() {
        let input = r#"
    witness {
        a: u232,
    }
"#;

        let expected = Ok(vec![Witness::new(
            Location::new(3, 9),
            Identifier::new(Location::new(3, 9), "a".to_owned()),
            Type::new(Location::new(3, 12), TypeVariant::new_integer_unsigned(232)),
        )]);

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_empty() {
        let input = r#"
    witness {}
"#;

        let expected = Ok(Vec::<Witness>::new());

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));;

        assert_eq!(expected, result);
    }

    #[test]
    fn err_expected_comma() {
        let input = r#"
    witness {
        a: u232;
    }
"#;

        let expected = Err(Error::Syntax(SyntaxError::Expected(
            Location::new(3, 16),
            vec![","],
            Lexeme::Symbol(Symbol::Semicolon),
        )));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }
}
