//!
//! The witnesses parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Field;
use crate::syntax::FieldListParser;

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordWitness,
    BracketCurlyLeft,
    FieldList,
    BracketCurlyRight,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordWitness
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    fields: Vec<Field>,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<Vec<Field>, Error> {
        loop {
            match self.state {
                State::KeywordWitness => {
                    let next = stream.borrow_mut().next()?;
                    match next {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Witness),
                            ..
                        } => self.state = State::BracketCurlyLeft,
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["witness"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::BracketCurlyLeft => {
                    let next = stream.borrow_mut().next()?;
                    match next {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            ..
                        } => self.state = State::FieldList,
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::FieldList => {
                    let (fields, next) = FieldListParser::default().parse(stream.clone(), None)?;
                    self.fields = fields;
                    self.next = next;
                    self.state = State::BracketCurlyRight;
                }
                State::BracketCurlyRight => {
                    match self.next.take().expect("Always contains a value") {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        } => return Ok(self.fields),
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["}"],
                                lexeme,
                            )));
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
    use crate::lexical::Location;
    use crate::lexical::TokenStream;
    use crate::syntax::Field;
    use crate::syntax::Identifier;
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;

    #[test]
    fn ok_single() {
        let input = r#"
    witness {
        a: u232,
    }
"#;

        let expected = Ok(vec![Field::new(
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

        let expected = Ok(Vec::<Field>::new());

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }
}
