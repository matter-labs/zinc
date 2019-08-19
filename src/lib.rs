//!
//! The Jab compiler library.
//!

mod lexical;
mod syntax;

pub use self::syntax::CircuitProgram;

use failure::Fail;
use serde_derive::Serialize;

use self::lexical::TokenStream;

#[derive(Debug, Fail, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[fail(display = "lexical error: {}", _0)]
    Lexical(lexical::Error),
    #[fail(display = "syntax error: {}", _0)]
    Syntax(syntax::Error),
}

pub type CircuitResult = Result<CircuitProgram, Error>;

pub fn compile(input: Vec<u8>) -> CircuitResult {
    syntax::parse(TokenStream::new(input))
}
