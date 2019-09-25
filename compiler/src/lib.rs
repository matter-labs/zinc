//!
//! The Jab compiler library.
//!

mod generator;
mod interpreter;
mod lexical;
mod syntax;

pub use self::syntax::CircuitProgram;

use std::path::PathBuf;

use failure::Fail;

use self::generator::Generator;
use self::interpreter::Interpreter;
use self::lexical::TokenStream;
use self::syntax::Parser;

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

pub fn parse(input: String) -> Result<CircuitProgram, Error> {
    Parser::parse(TokenStream::new(input))
}

pub fn interpret(input: String) -> Result<(), Error> {
    Interpreter::default()
        .interpret(Parser::parse(TokenStream::new(input))?)
        .map_err(Error::Interpreter)
}

pub fn generate(input: String, output: PathBuf) -> Result<(), Error> {
    Generator::new(output)
        .generate(Parser::parse(TokenStream::new(input))?)
        .map_err(Error::Generator)
}
