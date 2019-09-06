//!
//! The Jab compiler library.
//!

mod interpreter;
mod lexical;
mod syntax;

pub use self::syntax::CircuitProgram;

use failure::Fail;
use serde_derive::Serialize;

use self::interpreter::Interpreter;
use self::lexical::TokenStream;
use self::syntax::Parser;

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
    Parser::parse(TokenStream::new(input))
}

pub fn interpret(program: CircuitProgram) -> Result<(), Error> {
    Interpreter::interpret(program).map_err(Error::Semantic)
}
