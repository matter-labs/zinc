//!
//! The string parser.
//!

use std::str;

use failure::Fail;
use serde_derive::Serialize;

pub enum State {
    DoubleQuoteOpen,
    Character,
    DoubleQuoteClose,
}

#[derive(Debug, Fail, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[fail(display = "unexpected end")]
    UnexpectedEnd,
    #[fail(display = "not a string")]
    NotAString,
}

pub fn parse(bytes: &[u8]) -> Result<(usize, String), Error> {
    let mut state = State::DoubleQuoteOpen;
    let mut size = 0;

    while let Some(byte) = bytes.get(size).copied() {
        match state {
            State::DoubleQuoteOpen => match byte {
                b'\"' => state = State::Character,
                _ => return Err(Error::NotAString),
            },
            State::Character => {
                if let b'\"' = byte {
                    state = State::DoubleQuoteClose;
                }
            }
            State::DoubleQuoteClose => {
                let string = unsafe { str::from_utf8_unchecked(&bytes[1..size - 1]) }.to_owned();
                return Ok((size, string));
            }
        }

        size += 1;
    }

    Err(Error::UnexpectedEnd)
}
