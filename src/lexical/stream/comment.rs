//!
//! The comment lexical parser.
//!

use failure::Fail;
use serde_derive::Serialize;

use crate::lexical::Comment;

pub enum State {
    Start,
    Slash,
    SingleLine,
    MultiLine,
    MultiLineStar,
}

#[derive(Debug, Fail, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[fail(display = "not a comment")]
    NotAComment,
}

pub fn parse(bytes: &[u8]) -> Result<(usize, usize, Comment), Error> {
    let mut state = State::Start;
    let mut size = 0;
    let mut lines = 0;

    while let Some(byte) = bytes.get(size).copied() {
        match state {
            State::Start => match byte {
                b'/' => state = State::Slash,
                _ => return Err(Error::NotAComment),
            },
            State::Slash => match byte {
                b'/' => state = State::SingleLine,
                b'*' => state = State::MultiLine,
                _ => return Err(Error::NotAComment),
            },
            State::SingleLine => {
                if b'\n' == byte {
                    break;
                }
            }
            State::MultiLine => match byte {
                b'*' => state = State::MultiLineStar,
                b'\n' => lines += 1,
                _ => {}
            },
            State::MultiLineStar => match byte {
                b'/' => {
                    size += 1;
                    break;
                }
                _ => state = State::MultiLine,
            },
        }

        size += 1;
    }

    let comment = Comment(String::from_utf8_lossy(&bytes[..size]).to_string());
    Ok((size, lines, comment))
}
