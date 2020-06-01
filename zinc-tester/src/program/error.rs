//!
//! The Zinc tester program error.
//!

use failure::Fail;

use zinc_bytecode::TemplateValueError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "compiler: {}", _0)]
    Compiler(String),
    #[fail(display = "program: {}", _0)]
    Program(String),
    #[fail(display = "template value: {}", _0)]
    TemplateValue(TemplateValueError),
    #[fail(display = "entry not found: {}", _0)]
    EntryNotFound(String),
}
