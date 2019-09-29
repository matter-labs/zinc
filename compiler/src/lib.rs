//!
//! The compiler library.
//!

#![allow(clippy::large_enum_variant)]
#![allow(clippy::should_implement_trait)]

mod error;
mod generator;
mod interpreter;
mod lexical;
mod syntax;

pub use self::error::Error;
pub use self::syntax::CircuitProgram;

use std::path::PathBuf;

use self::generator::Generator;
use self::interpreter::Interpreter;
use self::lexical::TokenStream;
use self::syntax::Parser;

pub const SIZE_BYTE: usize = 8;
pub const SIZE_MAX_INT: usize = 248;
pub const SIZE_FIELD: usize = 254;
pub const SIZE_FIELD_PADDED: usize = 256;
pub const MAX_BYTE: usize = 256;
pub const BASE_DECIMAL: usize = 10;
pub const BASE_HEXADECIMAL: usize = 16;

pub fn parse(input: String) -> Result<CircuitProgram, Error> {
    Parser::parse(TokenStream::new(input))
}

pub fn interpret(circuit: CircuitProgram) -> Result<(), Error> {
    Ok(Interpreter::default().interpret(circuit)?)
}

pub fn generate(circuit: CircuitProgram, output: PathBuf) -> Result<(), Error> {
    Ok(Generator::new(output)?.generate(circuit)?)
}
