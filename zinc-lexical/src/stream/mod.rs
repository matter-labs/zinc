//!
//! The lexical token stream.
//!

pub mod comment;
pub mod integer;
pub mod string;
pub mod symbol;
pub mod word;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::error::Error;
use crate::token::lexeme::comment::Comment;
use crate::token::lexeme::identifier::Identifier;
use crate::token::lexeme::literal::string::String as StringLiteral;
use crate::token::lexeme::literal::Literal;
use crate::token::lexeme::Lexeme;
use crate::token::location::Location;
use crate::token::Token;

use self::comment::error::Error as CommentParserError;
use self::integer::error::Error as IntegerParserError;
use self::string::error::Error as StringParserError;
use self::symbol::error::Error as SymbolParserError;

///
/// A token stream is initialized for each input file.
///
pub struct TokenStream<'a> {
    /// The input source code string reference
    input: &'a str,
    /// The number of characters processed so far
    offset: usize,
    /// The current position in the file
    location: Location,
    /// The queue buffer where the characters acquired with the look-ahead method are stored.
    /// If the queue is not empty, the next character will be taken therefrom.
    look_ahead: VecDeque<Token>,
}

impl<'a> TokenStream<'a> {
    /// The initial capacity of the look-ahead buffer queue.
    const LOOK_AHEAD_INITIAL_CAPACITY: usize = 16;

    ///
    /// Initializes a stream with a file identifier.
    /// The file identifier can be used to get its path from the global type index.
    ///
    pub fn new(input: &'a str, file: usize) -> Self {
        Self {
            input,
            offset: 0,
            location: Location::new(file),
            look_ahead: VecDeque::with_capacity(Self::LOOK_AHEAD_INITIAL_CAPACITY),
        }
    }

    ///
    /// Wraps the stream into `Rc<RefCell<_>>` simplifying most of initializations.
    ///
    pub fn wrap(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }

    ///
    /// Picks a character from the look-ahead queue.
    /// If the queue is empty, advances the stream iterator.
    ///
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Result<Token, Error> {
        let token = match self.look_ahead.pop_front() {
            Some(token) => token,
            None => self.advance()?,
        };
        log::debug!("{:?}", token);
        Ok(token)
    }

    ///
    /// Advances the iterator until there are `distance` elements in the look-ahead queue.
    /// Is used where there is a need to resolve an ambiguity like `if value {}`,
    /// where `value` is able to start both a variable or structure literal type name.
    ///
    /// # Panics
    /// If the `distance` is zero
    ///
    pub fn look_ahead(&mut self, distance: usize) -> Result<&Token, Error> {
        while self.look_ahead.len() < distance {
            let token = self.advance()?;
            self.look_ahead.push_back(token);
        }

        self.look_ahead
            .get(distance - 1)
            .ok_or_else(|| Error::unexpected_end(self.location))
    }

    ///
    /// Initializes a stream with an auto-generated file identifier.
    /// The file identifier can be used to get its path from the global type index.
    /// Used for testing purposes.
    ///
    pub fn test(input: &'a str) -> Self {
        Self {
            input,
            offset: 0,
            location: Location::new(0),
            look_ahead: VecDeque::with_capacity(Self::LOOK_AHEAD_INITIAL_CAPACITY),
        }
    }

    ///
    /// The function checks if a character:
    /// 1. Is a whitespace -> skip
    /// 2. Starts a comment -> start the comment subparser
    /// 3. Starts a string literal -> start the string subparser
    /// 4. Starts a number -> start the number subparser
    /// 5. Starts a word -> start the word subparser
    /// 6. Starts a symbol -> start the operand subparser
    /// 7. Is unknown -> yield an 'invalid character' error
    ///
    /// If the end of input has been reached, an 'EOF' token is returned for consequent calls.
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
                    Ok(output) => {
                        self.location.line += output.lines;
                        self.location.column = match output.comment {
                            Comment::Line { .. } => 1,
                            Comment::Block { .. } => output.column,
                        };
                        self.offset += output.size;
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
                    Ok(output) => {
                        let location = self.location;
                        self.location.column += output.size;
                        self.offset += output.size;
                        return Ok(Token::new(
                            Lexeme::Literal(Literal::String(StringLiteral::new(output.string))),
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
                    Ok(output) => {
                        let location = self.location;
                        self.location.column += output.size;
                        self.offset += output.size;
                        return Ok(Token::new(
                            Lexeme::Literal(Literal::Integer(output.integer)),
                            location,
                        ));
                    }
                    Err(IntegerParserError::NotAnInteger) => {}
                    Err(IntegerParserError::EmptyBinaryBody { offset })
                    | Err(IntegerParserError::EmptyOctalBody { offset })
                    | Err(IntegerParserError::EmptyExponent { offset })
                    | Err(IntegerParserError::EmptyHexadecimalBody { offset }) => {
                        return Err(Error::unexpected_end(self.location.shifted_right(offset)));
                    }
                    Err(IntegerParserError::ExpectedOneOfBinary { found, offset }) => {
                        return Err(Error::expected_one_of_binary(
                            self.location.shifted_right(offset),
                            found,
                        ))
                    }
                    Err(IntegerParserError::ExpectedOneOfOctal { found, offset }) => {
                        return Err(Error::expected_one_of_octal(
                            self.location.shifted_right(offset),
                            found,
                        ))
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
                let output = self::word::parse(&self.input[self.offset..]);
                let location = self.location;
                self.location.column += output.size;
                self.offset += output.size;
                return Ok(Token::new(output.word, location));
            }

            return match self::symbol::parse(&self.input[self.offset..]) {
                Ok(output) => {
                    let location = self.location;
                    self.location.column += output.size;
                    self.offset += output.size;
                    Ok(Token::new(Lexeme::Symbol(output.symbol), location))
                }
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
