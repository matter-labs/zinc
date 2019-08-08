//!
//! The Jab compiler library.
//!

mod lexical;
mod syntax;

pub use self::lexical::Stream as LexicalStream;
pub use self::syntax::Analyzer as SyntaxAnalyzer;
pub use self::syntax::CircuitProgram;

use std::str::FromStr;

use failure::Fail;
use log::*;
use proc_macro2::TokenStream;
use serde_derive::Serialize;

#[derive(Debug, Fail, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[fail(display = "lexical: {}", _0)]
    Lexical(String),
    #[fail(display = "syntax: {}", _0)]
    Syntax(syntax::Error),
}

pub type CircuitResult = Result<CircuitProgram, Error>;

pub fn compile(input: String) -> CircuitResult {
    for result in LexicalStream::new(input.bytes().collect()) {
        match result {
            Ok(token) => trace!("Token: {}", token),
            Err(error) => {
                error!("Lexical error: {}", error);
                break;
            }
        }
    }

    let stream =
        TokenStream::from_str(&input).map_err(|error| Error::Lexical(format!("{:?}", error)))?;
    SyntaxAnalyzer::default()
        .analyze(stream)
        .map_err(Error::Syntax)
}
