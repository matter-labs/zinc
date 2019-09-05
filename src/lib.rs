//!
//! The Jab compiler library.
//!

mod interpreter;
mod lexical;
mod syntax;

pub use self::syntax::CircuitProgram;

use failure::Fail;
use serde_derive::Serialize;

use self::lexical::TokenStream;

#[derive(Debug, Fail, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[fail(display = "Lexical error: {}", _0)]
    Lexical(lexical::Error),
    #[fail(display = "Syntax error: {}", _0)]
    Syntax(syntax::Error),
    #[fail(display = "Semantic error: {}", _0)]
    Semantic(interpreter::Error),
}

pub fn parse(input: String) -> Result<CircuitProgram, Error> {
    syntax::parse(TokenStream::new(input))
}

pub fn interpret(program: CircuitProgram) -> Result<(), Error> {
    interpreter::interpret(program).map_err(Error::Semantic)
}
