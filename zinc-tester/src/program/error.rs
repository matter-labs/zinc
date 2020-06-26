//!
//! The Zinc tester program error.
//!

use failure::Fail;

use zinc_bytecode::TemplateValueError;

///
/// The test program error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The source code could not be compiled succesfully.
    #[fail(display = "compiler: {}", _0)]
    Compiler(String),
    /// The program could not be parsed from the bytecode bytes.
    #[fail(display = "program: {}", _0)]
    Program(String),
    /// The program input could not be filled from the test metadata.
    #[fail(display = "template value: {}", _0)]
    TemplateValue(TemplateValueError),
    /// The required entry could not be found in the test program.
    #[fail(display = "entry not found: {}", _0)]
    EntryNotFound(String),
}
