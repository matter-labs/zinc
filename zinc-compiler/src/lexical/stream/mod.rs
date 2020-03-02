//!
//! The lexical token stream.
//!

pub mod comment;
pub mod integer;
pub mod string;
pub mod symbol;
pub mod word;

use std::collections::VecDeque;

use crate::lexical::error::Error;
use crate::lexical::token::lexeme::identifier::Identifier;
use crate::lexical::token::lexeme::literal::string::String as StringLiteral;
use crate::lexical::token::lexeme::literal::Literal;
use crate::lexical::token::lexeme::Lexeme;
use crate::lexical::token::location::Location;
use crate::lexical::token::Token;

use self::comment::Error as CommentParserError;
use self::integer::Error as IntegerParserError;
use self::string::Error as StringParserError;
use self::symbol::Error as SymbolParserError;

pub struct TokenStream<'a> {
    input: &'a str,
    offset: usize,
    location: Location,
    look_ahead: VecDeque<Token>,
}

impl<'a> TokenStream<'a> {
    const DEQUE_LOOK_AHEAD_INITIAL_CAPACITY: usize = 16;

    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            offset: 0,
            location: Location::new_beginning(None),
            look_ahead: VecDeque::with_capacity(Self::DEQUE_LOOK_AHEAD_INITIAL_CAPACITY),
        }
    }

    pub fn new_with_file(input: &'a str, file: usize) -> Self {
        Self {
            input,
            offset: 0,
            location: Location::new_beginning(Some(file)),
            look_ahead: VecDeque::with_capacity(Self::DEQUE_LOOK_AHEAD_INITIAL_CAPACITY),
        }
    }

    pub fn next(&mut self) -> Result<Token, Error> {
        let token = match self.look_ahead.pop_front() {
            Some(token) => token,
            None => self.advance()?,
        };
        log::debug!("{:?}", token);
        Ok(token)
    }

    ///
    /// Advances the iterator until there is `distance` elements in the look-ahead queue.
    /// Is used where there is a need to resolve an ambiguity like
    /// ```
    /// let identifier = true;
    /// if identifier {
    ///     // value: 42, a structure literal field
    ///     let value = 42; // a statement within the block
    /// }
    /// ```
    /// where `identifier` can be both a variable or structure literal type name.
    ///
    pub fn look_ahead(&mut self, distance: usize) -> Result<&Token, Error> {
        while self.look_ahead.len() < distance {
            let token = self.advance()?;
            self.look_ahead.push_back(token);
        }
        Ok(&self.look_ahead[self.look_ahead.len() - 1])
    }

    ///
    /// The function algorithm checks if the character:
    /// 1. Is contained within the alphabet
    /// 2. Is a whitespace
    /// 3. Starts a comment
    /// 4. Starts a string literal
    /// 5. Starts a symbol (operator or delimiter)
    /// 6. Starts a number (decimal or hexadecimal)
    /// 7. Starts a word (keyword, boolean literal, or identifier)
    /// 8. Panics if non of the above, thus the alphabet must contain all the characters being
    /// passed to subscanners
    ///
    fn advance(&mut self) -> Result<Token, Error> {
        while let Some(character) = self.input.chars().nth(self.offset) {
            if character.is_ascii_whitespace() {
                if character == '\n' {
                    self.location.line += 1;
                    self.location.column = 1;
                } else if character != '\r' {
                    self.location.column += 1;
                }
                self.offset += 1;
                continue;
            }

            if character == '/' {
                match self::comment::parse(&self.input[self.offset..]) {
                    Ok((size, lines, column, _comment)) => {
                        self.location.line += lines;
                        self.location.column = column;
                        self.offset += size;
                        continue;
                    }
                    Err(CommentParserError::NotAComment) => {}
                    Err(CommentParserError::UnterminatedBlock { lines, column }) => {
                        return Err(Error::unterminated_block_comment(
                            self.location,
                            self.location.shifted_down(lines, column),
                        ));
                    }
                }
            }

            if character == '\"' {
                match self::string::parse(&self.input[self.offset..]) {
                    Ok((size, value)) => {
                        let location = self.location;
                        self.location.column += size;
                        self.offset += size;
                        return Ok(Token::new(
                            Lexeme::Literal(Literal::String(StringLiteral::new(value))),
                            location,
                        ));
                    }
                    Err(StringParserError::NotAString) => {}
                    Err(StringParserError::UnterminatedDoubleQuote { lines, column }) => {
                        return Err(Error::unterminated_double_quote_string(
                            self.location,
                            self.location.shifted_down(lines, column),
                        ));
                    }
                }
            }

            if character.is_ascii_digit() {
                match self::integer::parse(&self.input[self.offset..]) {
                    Ok((size, integer)) => {
                        let location = self.location;
                        self.location.column += size;
                        self.offset += size;
                        return Ok(Token::new(
                            Lexeme::Literal(Literal::Integer(integer)),
                            location,
                        ));
                    }
                    Err(IntegerParserError::NotAnInteger) => {}
                    Err(IntegerParserError::EmptyHexadecimalBody) => {
                        return Err(Error::unexpected_end(self.location));
                    }
                    Err(IntegerParserError::ExpectedOneOfDecimal { found, offset }) => {
                        return Err(Error::expected_one_of_decimal(
                            self.location.shifted_right(offset),
                            found,
                        ))
                    }
                    Err(IntegerParserError::ExpectedOneOfHexadecimal { found, offset }) => {
                        return Err(Error::expected_one_of_hexadecimal(
                            self.location.shifted_right(offset),
                            found,
                        ))
                    }
                    Err(IntegerParserError::UnexpectedEnd) => {
                        return Err(Error::unexpected_end(self.location))
                    }
                }
            }

            if Identifier::can_start_with(character) {
                let (size, lexeme) = self::word::parse(&self.input[self.offset..]);
                let location = self.location;
                self.location.column += size;
                self.offset += size;
                return Ok(Token::new(lexeme, location));
            }

            return match self::symbol::parse(&self.input[self.offset..]) {
                Ok((size, symbol)) => {
                    let location = self.location;
                    self.location.column += size;
                    self.offset += size;
                    Ok(Token::new(Lexeme::Symbol(symbol), location))
                }
                Err(SymbolParserError::ExpectedOneOf {
                    expected,
                    found,
                    offset,
                    ..
                }) => Err(Error::expected_one_of(
                    self.location.shifted_right(offset),
                    expected,
                    found,
                )),
                Err(SymbolParserError::InvalidCharacter { found, offset }) => Err(
                    Error::invalid_character(self.location.shifted_right(offset), found),
                ),
                Err(SymbolParserError::UnexpectedEnd) => {
                    Err(Error::unexpected_end(self.location.shifted_right(1)))
                }
            };
        }

        Ok(Token::new(Lexeme::Eof, self.location))
    }
}
