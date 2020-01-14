//!
//! The Zargo command error.
//!

use failure::Fail;

use crate::command::NewCommandError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "{}", _0)]
    New(NewCommandError),
}

impl From<NewCommandError> for Error {
    fn from(inner: NewCommandError) -> Self {
        Self::New(inner)
    }
}
