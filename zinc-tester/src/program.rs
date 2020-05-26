//!
//! The Zinc tester program.
//!

use std::collections::HashMap;

use failure::Fail;
use serde_json::Value as JsonValue;

use zinc_bytecode::Program as BytecodeProgram;
use zinc_bytecode::TemplateValue;
use zinc_bytecode::TemplateValueError;
use zinc_compiler::Bytecode;
use zinc_compiler::EntryAnalyzer;
use zinc_compiler::Error as CompilerError;
use zinc_compiler::Program as IntermediateProgram;
use zinc_compiler::Source;

pub struct ProgramData {
    pub program: BytecodeProgram,
    pub input: TemplateValue,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "compiler: {}", _0)]
    Compiler(String),
    #[fail(display = "program: {}", _0)]
    Program(String),
    #[fail(display = "template value: {}", _0)]
    TemplateValue(TemplateValueError),
}

impl ProgramData {
    pub fn new(witness: &JsonValue, code: &str) -> Result<Self, Error> {
        let program = Self::compile(code)?;
        let input = TemplateValue::from_typed_json(witness, &program.input)
            .map_err(Error::TemplateValue)?;

        Ok(Self { program, input })
    }

    pub fn compile(code: &str) -> Result<BytecodeProgram, Error> {
        let scope = EntryAnalyzer::define(Source::test(code, HashMap::new()))
            .map_err(CompilerError::Semantic)
            .map_err(|error| error.format())
            .map_err(Error::Compiler)?;

        let bytecode = Bytecode::new().wrap();
        IntermediateProgram::new(scope.borrow().get_intermediate())
            .write_all_to_bytecode(bytecode.clone());

        let main_entry = Bytecode::unwrap_rc(bytecode)
            .into_entries()
            .remove(zinc_compiler::FUNCTION_MAIN_IDENTIFIER)
            .expect(crate::panic::MAIN_ENTRY_ID);
        let program =
            BytecodeProgram::from_bytes(main_entry.bytecode.as_slice()).map_err(Error::Program)?;

        Ok(program)
    }
}
