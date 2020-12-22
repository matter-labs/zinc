//!
//! The Zargo package manager `verify` subcommand.
//!

use std::convert::TryFrom;
use std::path::PathBuf;

use structopt::StructOpt;

use crate::error::Error;
use crate::executable::virtual_machine::VirtualMachine;
use crate::project::data::Directory as DataDirectory;
use crate::project::target::deps::Directory as TargetDependenciesDirectory;
use crate::project::target::Directory as TargetDirectory;

///
/// The Zargo package manager `verify` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Verifies the zero-knowledge proof")]
pub struct Command {
    /// Prints more logs, if passed several times.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbosity: usize,

    /// Suppresses output, if set.
    #[structopt(short = "q", long = "quiet")]
    pub quiet: bool,

    /// The path to the Zinc project manifest file.
    #[structopt(
        long = "manifest-path",
        parse(from_os_str),
        default_value = "./Zargo.toml"
    )]
    pub manifest_path: PathBuf,

    /// The contract method to verify. Only for contracts.
    #[structopt(long = "method")]
    pub method: Option<String>,

    /// Uses the release build.
    #[structopt(long = "release")]
    pub is_release: bool,
}

impl Command {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        verbosity: usize,
        quiet: bool,
        manifest_path: PathBuf,
        method: Option<String>,
        is_release: bool,
    ) -> Self {
        Self {
            verbosity,
            quiet,
            manifest_path,
            method,
            is_release,
        }
    }

    ///
    /// Executes the command.
    ///
    pub fn execute(self) -> anyhow::Result<()> {
        let manifest = zinc_project::Manifest::try_from(&self.manifest_path)?;

        match manifest.project.r#type {
            zinc_project::ProjectType::Contract if self.method.is_none() => {
                anyhow::bail!(Error::MethodMissing)
            }
            _ => {}
        }

        let mut manifest_path = self.manifest_path.clone();
        if manifest_path.is_file() {
            manifest_path.pop();
        }

        let data_directory_path = DataDirectory::path(&manifest_path);
        let mut output_path = data_directory_path.clone();
        output_path.push(format!(
            "{}.{}",
            zinc_const::file_name::OUTPUT,
            zinc_const::extension::JSON,
        ));
        let mut verifying_key_path = data_directory_path;
        verifying_key_path.push(zinc_const::file_name::VERIFYING_KEY.to_owned());

        let target_directory_path = TargetDirectory::path(&manifest_path, self.is_release);
        let mut binary_path = target_directory_path;
        binary_path.push(format!(
            "{}.{}",
            zinc_const::file_name::BINARY,
            zinc_const::extension::BINARY
        ));
        TargetDependenciesDirectory::create(&manifest_path)?;

        match self.method {
            Some(method) => VirtualMachine::verify_contract(
                self.verbosity,
                self.quiet,
                &binary_path,
                &verifying_key_path,
                &output_path,
                method.as_str(),
            ),
            _ => VirtualMachine::verify_circuit(
                self.verbosity,
                self.quiet,
                &binary_path,
                &verifying_key_path,
                &output_path,
            ),
        }?;

        Ok(())
    }
}
