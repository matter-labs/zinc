//!
//! The Jab compiler library.
//!

mod interpreter;
mod lexical;
mod syntax;

pub use self::interpreter::Executor;
pub use self::interpreter::Field;
pub use self::syntax::CircuitProgram;
pub use self::syntax::Expression;
pub use self::syntax::Statement;

use failure::Fail;
use serde_derive::Serialize;

use self::lexical::TokenStream;

#[derive(Debug, Fail, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[fail(display = "Lexical error: {}", _0)]
    Lexical(lexical::Error),
    #[fail(display = "Syntax error: {}", _0)]
    Syntax(syntax::Error),
    #[fail(display = "Semantic error: {}", _0)]
    Semantic(interpreter::Error),
}

pub fn compile(input: Vec<u8>) -> Result<CircuitProgram, Error> {
    syntax::parse(TokenStream::new(input))
}

pub fn execute(expression: Expression) -> Result<Field, Error> {
    Executor::default()
        .execute(expression)
        .map_err(Error::Semantic)
}
