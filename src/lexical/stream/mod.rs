//!
//! The token stream.
//!

mod comment;
mod integer;
mod symbol;
mod word;

pub use self::comment::parse as parse_comment;
pub use self::comment::Error as CommentParserError;
pub use self::integer::parse as parse_integer;
pub use self::integer::Error as IntegerParserError;
pub use self::symbol::parse as parse_symbol;
pub use self::symbol::Error as SymbolParserError;
pub use self::word::parse as parse_word;

use std::iter::Iterator;

use crate::lexical::Alphabet;
use crate::lexical::Error;
use crate::lexical::Identifier;
use crate::lexical::Lexeme;
use crate::lexical::Literal;
use crate::lexical::Location;
use crate::lexical::Token;

pub struct TokenStream {
    input: Vec<u8>,
    cursor: Cursor,
    peeked: Option<Result<Token, Error>>,
}

impl TokenStream {
    ///
    /// Initializes the stream from the beginning of `input`.
    ///
    pub fn new(input: Vec<u8>) -> Self {
        Self {
            input,
            cursor: Cursor::new(),
            peeked: None,
        }
    }

    ///
    /// Peeks the value, stores it in the `self.peeked` and returns a copy of it.
    ///
    pub fn peek(&mut self) -> Option<Result<Token, Error>> {
        if self.peeked.is_none() {
            self.peeked = self.advance();
        }
        self.peeked.clone()
    }

    fn advance(&mut self) -> Option<Result<Token, Error>> {
        while let Some(byte) = self.input.get(self.cursor.index).copied() {
            if !Alphabet::contains(byte) {
                let location = Location::new(self.cursor.line, self.cursor.column);
                return Some(Err(Error::InvalidCharacter(location, char::from(byte))));
            }

            if byte.is_ascii_whitespace() {
                if byte == b'\n' {
                    self.cursor.line += 1;
                    self.cursor.column = 1;
                } else if byte != b'\r' {
                    self.cursor.column += 1;
                }
                self.cursor.index += 1;
                continue;
            }

            if byte == b'/' {
                if let Ok((size, lines, column, _comment)) =
                    parse_comment(&self.input[self.cursor.index..])
                {
                    self.cursor.line += lines;
                    self.cursor.column = column;
                    self.cursor.index += size;
                    continue;
                }
            }

            match parse_symbol(&self.input[self.cursor.index..]) {
                Ok((size, symbol)) => {
                    let location = Location::new(self.cursor.line, self.cursor.column);
                    self.cursor.column += size;
                    self.cursor.index += size;
                    return Some(Ok(Token::new(Lexeme::Symbol(symbol), location)));
                }
                Err(SymbolParserError::NotASymbol) => {}
                Err(error) => {
                    let location = Location::new(self.cursor.line, self.cursor.column);
                    return Some(Err(Error::InvalidSymbol(location, error)));
                }
            }

            if Identifier::can_start_with(byte) {
                let (size, lexeme) = parse_word(&self.input[self.cursor.index..]);
                let location = Location::new(self.cursor.line, self.cursor.column);
                self.cursor.column += size;
                self.cursor.index += size;
                return Some(Ok(Token::new(lexeme, location)));
            }

            if byte.is_ascii_digit() {
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
                    Err(error) => {
                        let location = Location::new(self.cursor.line, self.cursor.column);
                        return Some(Err(Error::InvalidIntegerLiteral(location, error)));
                    }
                }
            }

            unreachable!();
        }

        None
    }
}

impl Iterator for TokenStream {
    type Item = Result<Token, Error>;

    ///
    /// Returns the next token from the stream.
    ///
    /// If there is a peeked value, it is returned, otherwise the stream is advanced.
    ///
    fn next(&mut self) -> Option<Self::Item> {
        match self.peeked.take() {
            Some(peeked) => Some(peeked),
            None => self.advance(),
        }
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
