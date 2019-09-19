//!
//! The Jab compiler library.
//!

mod executor;
mod lexical;
mod syntax;

pub use self::syntax::CircuitProgram;

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use failure::Fail;
use serde_derive::Serialize;

use self::executor::Generator;
use self::executor::Interpreter;
use self::executor::Scope;
use self::executor::Writer;
use self::lexical::TokenStream;
use self::syntax::Parser;

#[derive(Debug, Fail, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[fail(display = "Lexical error: {}", _0)]
    Lexical(lexical::Error),
    #[fail(display = "Syntax error: {}", _0)]
    Syntax(syntax::Error),
    #[fail(display = "Executor error: {}", _0)]
    Executor(executor::Error),
}

pub fn parse(input: String) -> Result<CircuitProgram, Error> {
    Parser::parse(TokenStream::new(input))
}

pub fn interpret(program: CircuitProgram) -> Result<(), Error> {
    let mut interpreter = Interpreter::new(Scope::new(None));
    interpreter
        .interpret(program.clone())
        .map_err(Error::Executor)
}

pub fn generate(program: CircuitProgram) -> Result<(), Error> {
    let mut writer = Writer::new(Rc::new(RefCell::new(Generator::new(PathBuf::from(
        "circuit/src/lib.rs",
    )))));
    writer.translate(program).map_err(Error::Executor)
}
