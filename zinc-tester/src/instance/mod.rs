//!
//! The Zinc tester instance.
//!

pub mod error;

use std::collections::HashMap;
use std::path::PathBuf;
use std::thread;

use serde_json::Value as JsonValue;

use zinc_build::Program as BuildProgram;
use zinc_build::Value as BuildValue;
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
    pub witness: BuildValue,
    /// The instance bytecode with metadata.
    pub program: BuildProgram,
}

impl Instance {
    ///
    /// Creates a test instance.
    ///
    pub fn new(
        name: String,
        code: &str,
        path: PathBuf,
        method: String,
        witness: JsonValue,
    ) -> Result<Self, Error> {
        let source = Source::test(code, path, HashMap::new())
            .map_err(|error| Error::Compiler(format!("{:?}", error)))?;
        let program = thread::Builder::new()
            .stack_size(zinc_const::limit::COMPILER_STACK_SIZE)
            .spawn(|| {
                let scope = EntryAnalyzer::define(source)
                    .map_err(CompilerError::Semantic)
                    .map_err(|error| format!("{:?}", error))
                    .map_err(Error::Compiler)?;

                let state = State::new(name).wrap();
                IntermediateProgram::new(scope.borrow().get_intermediate())
                    .write_all(state.clone());

                Ok(State::unwrap_rc(state).into_program(true))
            })
            .expect(zinc_const::panic::SYNCHRONIZATION)
            .join()
            .expect(zinc_const::panic::SYNCHRONIZATION)?;

        let input_type = match program {
            BuildProgram::Circuit(ref circuit) => circuit.input.to_owned(),
            BuildProgram::Contract(ref contract) => contract
                .methods
                .get(method.as_str())
                .ok_or(Error::MethodNotFound(method))?
                .input
                .to_owned(),
        };

        let witness =
            BuildValue::try_from_typed_json(witness, input_type).map_err(Error::InputValue)?;

        Ok(Self { witness, program })
    }
}
