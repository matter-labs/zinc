//!
//! The compiler error.
//!

use failure::Fail;

use crate::generator;
use crate::interpreter;
use crate::lexical;
use crate::syntax;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "Lexical error: {}", _0)]
    Lexical(lexical::Error),
    #[fail(display = "Syntax error: {}", _0)]
    Syntax(syntax::Error),
    #[fail(display = "Interpreter error: {}", _0)]
    Interpreter(interpreter::Error),
    #[fail(display = "Generator error: {}", _0)]
    Generator(generator::Error),
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

impl From<interpreter::Error> for Error {
    fn from(error: interpreter::Error) -> Self {
        Self::Interpreter(error)
    }
}

impl From<generator::Error> for Error {
    fn from(error: generator::Error) -> Self {
        Self::Generator(error)
    }
}
