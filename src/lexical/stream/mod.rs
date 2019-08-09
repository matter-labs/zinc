//!
//! The lexeme stream.
//!

mod comment;
mod integer;
mod operator;
mod word;

pub use self::comment::parse as parse_comment;
pub use self::comment::Error as CommentParserError;
pub use self::integer::parse as parse_integer;
pub use self::integer::Error as IntegerParserError;
pub use self::operator::parse as parse_operator;
pub use self::operator::Error as OperatorParserError;
pub use self::word::parse as parse_word;

use std::convert::TryFrom;
use std::iter::Iterator;

use crate::lexical::Alphabet;
use crate::lexical::Delimiter;
use crate::lexical::Error;
use crate::lexical::Identifier;
use crate::lexical::Lexeme;
use crate::lexical::Literal;
use crate::lexical::Location;
use crate::lexical::Punctuation;
use crate::lexical::Token;

pub struct TokenStream {
    input: Vec<u8>,
    position: usize,
    line: usize,
    column: usize,
}

impl TokenStream {
    pub fn new(input: Vec<u8>) -> Self {
        Self {
            input,
            position: 0,
            line: 1,
            column: 1,
        }
    }
}

impl Iterator for TokenStream {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(byte) = self.input.get(self.position).copied() {
            if !Alphabet::contains(byte) {
                let location = Location::new(self.line, self.column);
                return Some(Err(Error::Forbidden(location, char::from(byte))));
            }

            if byte.is_ascii_whitespace() {
                if byte == b'\n' {
                    self.line += 1;
                    self.column = 1;
                } else if byte != b'\r' {
                    self.column += 1;
                }
                self.position += 1;
                continue;
            }

            if byte == b'/' {
                if let Ok((size, lines, _comment)) = parse_comment(&self.input[self.position..]) {
                    self.line += lines;
                    self.column += size;
                    self.position += size;
                    continue;
                }
            }

            if let Ok(punctuation) = Punctuation::try_from(byte) {
                let location = Location::new(self.line, self.column);
                self.column += 1;
                self.position += 1;
                return Some(Ok(Token::new(Lexeme::Punctuation(punctuation), location)));
            }

            if let Ok(delimiter) = Delimiter::try_from(byte) {
                let location = Location::new(self.line, self.column);
                self.column += 1;
                self.position += 1;
                return Some(Ok(Token::new(Lexeme::Delimiter(delimiter), location)));
            }

            match parse_operator(&self.input[self.position..]) {
                Ok((size, operator)) => {
                    let location = Location::new(self.line, self.column);
                    self.column += size;
                    self.position += size;
                    return Some(Ok(Token::new(Lexeme::Operator(operator), location)));
                }
                Err(OperatorParserError::NotAnOperator) => {}
                Err(error) => {
                    let location = Location::new(self.line, self.column);
                    return Some(Err(Error::InvalidOperator(location, error)));
                }
            }

            if Identifier::can_start_with(byte) {
                let (size, lexeme) = parse_word(&self.input[self.position..]);
                let location = Location::new(self.line, self.column);
                self.column += size;
                self.position += size;
                return Some(Ok(Token::new(lexeme, location)));
            }

            if byte.is_ascii_digit() {
                match parse_integer(&self.input[self.position..]) {
                    Ok((size, integer)) => {
                        let location = Location::new(self.line, self.column);
                        self.column += size;
                        self.position += size;
                        return Some(Ok(Token::new(
                            Lexeme::Literal(Literal::Integer(integer)),
                            location,
                        )));
                    }
                    Err(error) => {
                        let location = Location::new(self.line, self.column);
                        return Some(Err(Error::InvalidIntegerLiteral(location, error)));
                    }
                }
            }

            unreachable!();
        }

        None
    }
}
