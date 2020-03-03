//!
//! The Zinc tester program.
//!

use failure::Fail;
use serde_json::Value as JsonValue;

use zinc_bytecode::data::values::JsonValueError;
use zinc_bytecode::data::values::Value;
use zinc_bytecode::program::Program;

pub struct ProgramData {
    pub program: Program,
    pub input: Value,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "compiler: {}", _0)]
    Compiler(String),
    #[fail(display = "program: {}", _0)]
    Program(String),
    #[fail(display = "JSON type value: {}", _0)]
    JsonTypeValue(JsonValueError),
}

impl ProgramData {
    pub fn new(witness: &JsonValue, code: &str) -> Result<Self, Error> {
        let program = compile(code)?;
        let input =
            Value::from_typed_json(witness, &program.input).map_err(Error::JsonTypeValue)?;

        Ok(Self { program, input })
    }
}

pub fn compile(code: &str) -> Result<Program, Error> {
    let bytecode = zinc_compiler::compile_test(code).map_err(Error::Compiler)?;
    let bytecode: Vec<u8> = bytecode.into();
    let program = Program::from_bytes(bytecode.as_slice()).map_err(Error::Program)?;

    Ok(program)
}
