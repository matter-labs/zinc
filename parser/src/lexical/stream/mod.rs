//!
//! The lexical token stream.
//!

mod comment;
mod integer;
mod string;
mod symbol;
mod word;

pub use self::comment::parse as parse_comment;
pub use self::comment::Error as CommentParserError;
pub use self::integer::parse as parse_integer;
pub use self::integer::Error as IntegerParserError;
pub use self::string::parse as parse_string;
pub use self::string::Error as StringParserError;
pub use self::symbol::parse as parse_symbol;
pub use self::symbol::Error as SymbolParserError;
pub use self::word::parse as parse_word;
pub use self::word::Error as WordParserError;

use std::iter::Iterator;

use crate::lexical::Alphabet;
use crate::lexical::Error;
use crate::lexical::Identifier;
use crate::lexical::Lexeme;
use crate::lexical::Literal;
use crate::lexical::Location;
use crate::lexical::StringLiteral;
use crate::lexical::Token;

pub struct TokenStream {
    input: String,
    cursor: Cursor,
    peeked: Option<Result<Token, Error>>,
}

impl TokenStream {
    pub fn new(input: String) -> Self {
        Self {
            input,
            cursor: Cursor::new(),
            peeked: None,
        }
    }

    pub fn peek(&mut self) -> Option<Result<Token, Error>> {
        if self.peeked.is_none() {
            self.peeked = self.advance();
        }
        self.peeked.clone()
    }

    pub fn location(&self) -> Location {
        Location::new(self.cursor.line, self.cursor.column)
    }

    fn advance(&mut self) -> Option<Result<Token, Error>> {
        while let Some(character) = self.input.chars().nth(self.cursor.index) {
            if !Alphabet::contains(character) {
                let location = Location::new(self.cursor.line, self.cursor.column);
                return Some(Err(Error::InvalidCharacter(location, character)));
            }

            if character.is_ascii_whitespace() {
                if character == '\n' {
                    self.cursor.line += 1;
                    self.cursor.column = 1;
                } else if character != '\r' {
                    self.cursor.column += 1;
                }
                self.cursor.index += 1;
                continue;
            }

            if character == '/' {
                match parse_comment(&self.input[self.cursor.index..]) {
                    Ok((size, lines, column, _comment)) => {
                        self.cursor.line += lines;
                        self.cursor.column = column;
                        self.cursor.index += size;
                        continue;
                    }
                    Err(CommentParserError::UnexpectedEnd) => {
                        let location = Location::new(self.cursor.line, self.cursor.column);
                        return Some(Err(Error::UnexpectedEnd(location)));
                    }
                    Err(CommentParserError::NotAComment) => {}
                }
            }

            if character == '\"' {
                match parse_string(&self.input[self.cursor.index..]) {
                    Ok((size, value)) => {
                        let location = Location::new(self.cursor.line, self.cursor.column);
                        self.cursor.column += size;
                        self.cursor.index += size;
                        return Some(Ok(Token::new(
                            Lexeme::Literal(Literal::String(StringLiteral::new(value))),
                            location,
                        )));
                    }
                    Err(StringParserError::UnexpectedEnd) => {
                        let location = Location::new(self.cursor.line, self.cursor.column);
                        return Some(Err(Error::UnexpectedEnd(location)));
                    }
                    Err(StringParserError::NotAString) => {}
                }
            }

            match parse_symbol(&self.input[self.cursor.index..]) {
                Ok((size, symbol)) => {
                    let location = Location::new(self.cursor.line, self.cursor.column);
                    self.cursor.column += size;
                    self.cursor.index += size;
                    return Some(Ok(Token::new(Lexeme::Symbol(symbol), location)));
                }
                Err(SymbolParserError::UnexpectedEnd) => {
                    let location = Location::new(self.cursor.line, self.cursor.column);
                    return Some(Err(Error::UnexpectedEnd(location)));
                }
                Err(SymbolParserError::NotASymbol) => {}
                Err(error) => {
                    let location = Location::new(self.cursor.line, self.cursor.column);
                    return Some(Err(Error::InvalidSymbol(location, error)));
                }
            }

            if character.is_ascii_digit() {
                match parse_integer(&self.input[self.cursor.index..]) {
                    Ok((size, integer)) => {
                        let location = Location::new(self.cursor.line, self.cursor.column);
                        self.cursor.column += size;
                        self.cursor.index += size;
                        return Some(Ok(Token::new(
                            Lexeme::Literal(Literal::Integer(integer)),
                            location,
                        )));
                    }
                    Err(IntegerParserError::UnexpectedEnd) => {
                        let location = Location::new(self.cursor.line, self.cursor.column);
                        return Some(Err(Error::UnexpectedEnd(location)));
                    }
                    Err(IntegerParserError::NotAnInteger) => {}
                    Err(error) => {
                        let location = Location::new(self.cursor.line, self.cursor.column);
                        return Some(Err(Error::InvalidIntegerLiteral(location, error)));
                    }
                }
            }

            if Identifier::can_start_with(character) {
                match parse_word(&self.input[self.cursor.index..]) {
                    Ok((size, lexeme)) => {
                        let location = Location::new(self.cursor.line, self.cursor.column);
                        self.cursor.column += size;
                        self.cursor.index += size;
                        return Some(Ok(Token::new(lexeme, location)));
                    }
                    Err(error) => {
                        let location = Location::new(self.cursor.line, self.cursor.column);
                        return Some(Err(Error::InvalidWord(location, error)));
                    }
                }
            }

            panic!("Always checked by the branches above");
        }

        None
    }
}

impl Iterator for TokenStream {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = match self.peeked.take() {
            Some(peeked) => Some(peeked),
            None => self.advance(),
        };
        log::trace!("Token: {:?}", item);
        item
    }
}

#[derive(Clone, Copy)]
struct Cursor {
    index: usize,
    line: usize,
    column: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            index: 0,
            line: 1,
            column: 1,
        }
    }
}
