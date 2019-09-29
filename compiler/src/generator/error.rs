//!
//! The generator error.
//!

use failure::Fail;

use crate::generator::WriterError;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "writer: {}", _0)]
    Writer(WriterError),
}

impl From<WriterError> for Error {
    fn from(error: WriterError) -> Self {
        Self::Writer(error)
    }
}
