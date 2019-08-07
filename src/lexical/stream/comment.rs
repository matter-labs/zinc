//!
//! The comment lexical analyzer.
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

impl Default for State {
    fn default() -> Self {
        State::Start
    }
}

#[derive(Default)]
pub struct Analyzer {
    state: State,
}

#[derive(Debug, Fail, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[fail(display = "not a comment")]
    NotAComment,
}

impl Analyzer {
    pub fn analyze(mut self, bytes: &[u8]) -> Result<(usize, usize, Comment), Error> {
        let mut size = 0;
        let mut lines = 0;
        while let Some(byte) = bytes.get(size) {
            let byte = *byte;

            match self.state {
                State::Start => match byte {
                    b'/' => self.state = State::Slash,
                    _ => return Err(Error::NotAComment),
                },
                State::Slash => match byte {
                    b'/' => self.state = State::SingleLine,
                    b'*' => self.state = State::MultiLine,
                    _ => return Err(Error::NotAComment),
                },
                State::SingleLine => {
                    if b'\n' == byte {
                        break;
                    }
                }
                State::MultiLine => match byte {
                    b'*' => self.state = State::MultiLineStar,
                    b'\n' => lines += 1,
                    _ => {}
                },
                State::MultiLineStar => match byte {
                    b'/' => {
                        size += 1;
                        break;
                    }
                    _ => self.state = State::MultiLine,
                },
            }

            size += 1;
        }

        let comment = Comment(String::from_utf8_lossy(&bytes[..size]).to_string());
        Ok((size, lines, comment))
    }
}
