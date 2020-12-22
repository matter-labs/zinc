//!
//! The Zinc tester instance.
//!

use std::collections::HashMap;
use std::path::PathBuf;
use std::thread;

use anyhow::Context;

use zinc_compiler::EntryAnalyzer;
use zinc_compiler::Error as CompilerError;
use zinc_compiler::IBytecodeWritable;
use zinc_compiler::Source;
use zinc_compiler::ZincVMState;

use crate::error::Error;

///
/// The compiled Zinc instance.
///
#[derive(Debug)]
pub struct Instance {
    /// The input data template value.
    pub input: zinc_types::Value,
    /// The instance bytecode with metadata.
    pub application: zinc_types::Application,
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
            zinc_project::ProjectType::Contract
        } else {
            zinc_project::ProjectType::Circuit
        };

        let source = Source::test(code, path.clone(), HashMap::new())
            .with_context(|| path.to_string_lossy().to_string())?;

        let application = thread::Builder::new()
            .stack_size(zinc_const::limit::COMPILER_STACK_SIZE)
            .spawn(move || -> anyhow::Result<zinc_types::Application> {
                let project = zinc_project::ManifestProject::new(
                    name.clone(),
                    project_type,
                    semver::Version::new(1, 0, 0),
                );

                let scope = EntryAnalyzer::define(source, project, HashMap::new(), false)
                    .map_err(CompilerError::Semantic)
                    .map_err(|error| anyhow::anyhow!(error.format()))?;

                let state =
                    ZincVMState::new(zinc_project::Manifest::new(name.as_str(), project_type))
                        .wrap();
                zinc_compiler::Module::new(scope.borrow().get_intermediate())
                    .write_to_zinc_vm(state.clone());

                Ok(ZincVMState::unwrap_rc(state).into_application(true))
            })
            .expect(zinc_const::panic::SYNCHRONIZATION)
            .join()
            .expect(zinc_const::panic::SYNCHRONIZATION)?;

        let input_type = match application {
            zinc_types::Application::Circuit(ref circuit) => circuit.input.to_owned(),
            zinc_types::Application::Contract(ref contract) => {
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
            zinc_types::Application::Library(_library) => anyhow::bail!(Error::CannotRunLibrary),
        };

        let input = zinc_types::Value::try_from_typed_json(input, input_type)?;

        Ok(Self { input, application })
    }
}
