//!
//! The generator error.
//!

use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "file: {}", _0)]
    File(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::File(error)
    }
}

impl PartialEq<Self> for Error {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
