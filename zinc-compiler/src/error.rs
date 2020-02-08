//!
//! The Zinc compiler error.
//!

use failure::Fail;

use crate::lexical;
use crate::semantic;
use crate::syntax;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Lexical(lexical::Error),
    #[fail(display = "{}", _0)]
    Syntax(syntax::Error),
    #[fail(display = "{}", _0)]
    Semantic(semantic::Error),
}

impl From<lexical::Error> for Error {
    fn from(error: lexical::Error) -> Self {
        Self::Lexical(error)
    }
}

impl From<syntax::Error> for Error {
    fn from(error: syntax::Error) -> Self {
        Self::Syntax(error)
    }
}

impl From<semantic::Error> for Error {
    fn from(error: semantic::Error) -> Self {
        Self::Semantic(error)
    }
}
