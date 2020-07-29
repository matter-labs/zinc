//!
//! The Zinc tester instance.
//!

pub mod error;

use std::collections::HashMap;
use std::path::PathBuf;

use serde_json::Value as JsonValue;

use zinc_bytecode::Program as BytecodeProgram;
use zinc_bytecode::TemplateValue;
use zinc_compiler::EntryAnalyzer;
use zinc_compiler::Error as CompilerError;
use zinc_compiler::IBytecodeWritable;
use zinc_compiler::Module as IntermediateProgram;
use zinc_compiler::Source;
use zinc_compiler::State;

use self::error::Error;

///
/// The compiled Zinc instance.
///
pub struct Instance {
    /// The witness input data template value.
    pub witness: TemplateValue,
    /// The instance bytecode with metadata.
    pub program: BytecodeProgram,
}

impl Instance {
    ///
    /// Creates a test instance instance.
    ///
    pub fn new(
        name: String,
        code: &str,
        path: PathBuf,
        entry: String,
        witness: JsonValue,
    ) -> Result<Self, Error> {
        let scope = EntryAnalyzer::define(Source::test(code, path, 0, HashMap::new()))
            .map_err(CompilerError::Semantic)
            .map_err(|error| error.format())
            .map_err(Error::Compiler)?;

        let bytecode = State::new(name).wrap();
        IntermediateProgram::new(scope.borrow().get_intermediate()).write_all(bytecode.clone());

        let entry = State::unwrap_rc(bytecode)
            .into_entries(true)
            .remove(entry.as_str())
            .ok_or_else(|| Error::EntryNotFound(entry))?;

        let program = BytecodeProgram::from_bytes(entry.into_bytecode().as_slice())
            .map_err(Error::Program)?;

        let witness = TemplateValue::try_from_typed_json(witness, program.input())
            .map_err(Error::TemplateValue)?;

        Ok(Self { witness, program })
    }
}
