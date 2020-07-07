//!
//! The Zinc tester program.
//!

pub mod error;

use std::collections::HashMap;
use std::path::PathBuf;

use serde_json::Value as JsonValue;

use zinc_bytecode::Program as BytecodeProgram;
use zinc_bytecode::TemplateValue;
use zinc_compiler::Bytecode;
use zinc_compiler::EntryAnalyzer;
use zinc_compiler::Error as CompilerError;
use zinc_compiler::Program as IntermediateProgram;
use zinc_compiler::Source;

use self::error::Error;

///
/// The compiled Zinc program.
///
pub struct Program {
    /// The witness input data template value.
    pub witness: TemplateValue,
    /// The program bytecode with metadata.
    pub bytecode: BytecodeProgram,
}

impl Program {
    ///
    /// Creates a test program instance.
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

        let bytecode = Bytecode::new(name).wrap();
        IntermediateProgram::new(scope.borrow().get_intermediate())
            .write_all_to_bytecode(bytecode.clone());

        let entry = Bytecode::unwrap_rc(bytecode)
            .into_entries()
            .remove(entry.as_str())
            .ok_or_else(|| Error::EntryNotFound(entry))?;

        let bytecode = BytecodeProgram::from_bytes(entry.into_bytecode().as_slice())
            .map_err(Error::Program)?;

        let witness = TemplateValue::from_typed_json(witness, bytecode.input())
            .map_err(Error::TemplateValue)?;

        Ok(Self { witness, bytecode })
    }
}
