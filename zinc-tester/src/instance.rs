//!
//! The Zinc tester instance.
//!

use std::collections::HashMap;
use std::path::PathBuf;
use std::thread;

use anyhow::Context;

use zinc_build::Application as BuildApplication;
use zinc_compiler::EntryAnalyzer;
use zinc_compiler::Error as CompilerError;
use zinc_compiler::IBytecodeWritable;
use zinc_compiler::Module as IntermediateApplication;
use zinc_compiler::Source;
use zinc_compiler::State;
use zinc_manifest::Manifest;
use zinc_manifest::ProjectType;

use crate::error::Error;

///
/// The compiled Zinc instance.
///
pub struct Instance {
    /// The input data template value.
    pub input: zinc_build::Value,
    /// The instance bytecode with metadata.
    pub application: BuildApplication,
}

impl Instance {
    ///
    /// Creates a test instance.
    ///
    pub fn new(
        name: String,
        code: &str,
        path: PathBuf,
        method: Option<String>,
        input: serde_json::Value,
    ) -> anyhow::Result<Self> {
        let project_type = if method.is_some() {
            ProjectType::Contract
        } else {
            ProjectType::Circuit
        };

        let source = Source::test(code, path.clone(), HashMap::new())
            .with_context(|| path.to_string_lossy().to_string())?;

        let application = thread::Builder::new()
            .stack_size(zinc_const::limit::COMPILER_STACK_SIZE)
            .spawn(move || -> anyhow::Result<BuildApplication> {
                let scope = EntryAnalyzer::define(source)
                    .map_err(CompilerError::Semantic)
                    .map_err(|error| anyhow::anyhow!(error.format()))?;

                let state = State::new(Manifest::new(name.as_str(), project_type)).wrap();
                IntermediateApplication::new(scope.borrow().get_intermediate())
                    .write_all(state.clone());

                Ok(State::unwrap_rc(state).into_application(true))
            })
            .expect(zinc_const::panic::SYNCHRONIZATION)
            .join()
            .expect(zinc_const::panic::SYNCHRONIZATION)?;

        let input_type = match application {
            BuildApplication::Circuit(ref circuit) => circuit.input.to_owned(),
            BuildApplication::Contract(ref contract) => {
                let method = method
                    .ok_or(Error::MethodMissing)
                    .with_context(|| path.to_string_lossy().to_string())?;

                contract
                    .methods
                    .get(method.as_str())
                    .ok_or(Error::MethodNotFound(method))
                    .with_context(|| path.to_string_lossy().to_string())?
                    .input
                    .to_owned()
            }
        };

        let input = zinc_build::Value::try_from_typed_json(input, input_type)?;

        Ok(Self { input, application })
    }
}
