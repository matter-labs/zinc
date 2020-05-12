//!
//! The Zinc tester program.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use failure::Fail;
use serde_json::Value as JsonValue;

use zinc_bytecode::data::values::JsonValueError;
use zinc_bytecode::data::values::Value;
use zinc_bytecode::program::Program;
use zinc_compiler::Bytecode;
use zinc_compiler::EntryAnalyzer;
use zinc_compiler::Error as CompilerError;
use zinc_compiler::Source;

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
        let program = Self::compile(code)?;
        let input =
            Value::from_typed_json(witness, &program.input).map_err(Error::JsonTypeValue)?;

        Ok(Self { program, input })
    }

    pub fn compile(code: &str) -> Result<Program, Error> {
        let lines = code.lines().collect::<Vec<&str>>();

        let source = Source::test(code, HashMap::new())
            .map_err(|error| error.format(lines.as_slice()))
            .map_err(Error::Compiler)?;

        EntryAnalyzer::analyze(source)
            .map_err(CompilerError::Semantic)
            .map_err(|error| error.format(lines.as_slice()))
            .map_err(Error::Compiler)?;

        let bytecode = Rc::new(RefCell::new(Bytecode::new()));
        //        intermediate.write_all_to_bytecode(bytecode.clone());
        let mut bytecode = Rc::try_unwrap(bytecode)
            .expect(crate::PANIC_LAST_SHARED_REFERENCE)
            .into_inner();

        let entry_id = bytecode
            .function_name_to_entry_id(zinc_compiler::FUNCTION_MAIN_IDENTIFIER)
            .expect(crate::PANIC_MAIN_ENTRY_ID);
        let program = Program::from_bytes(bytecode.entry_to_bytes(entry_id).as_slice())
            .map_err(Error::Program)?;

        Ok(program)
    }
}
