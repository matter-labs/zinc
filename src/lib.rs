//!
//! The Jab compiler library.
//!

//pub mod examples;
//pub mod gadgets;

mod syntax;

use std::str::FromStr;

use failure::Fail;
use proc_macro2::TokenStream;

use self::syntax::Analyzer;
use self::syntax::CircuitProgram;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Lexical: {}", _0)]
    Lexical(String),
    #[fail(display = "Syntax: {}", _0)]
    Syntax(syntax::Error),
}

pub fn compile(input: &str) -> Result<CircuitProgram, Error> {
    let stream =
        TokenStream::from_str(&input).map_err(|error| Error::Lexical(format!("{:?}", error)))?;
    Analyzer::new().analyze(stream).map_err(Error::Syntax)
}
