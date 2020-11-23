//!
//! The Zargo package manager `proof-check` subcommand.
//!

use std::convert::TryFrom;
use std::path::PathBuf;

use structopt::StructOpt;

use crate::error::Error;
use crate::executable::compiler::Compiler;
use crate::executable::virtual_machine::VirtualMachine;
use crate::project::data::private_key::PrivateKey as PrivateKeyFile;
use crate::project::data::Directory as DataDirectory;
use crate::project::target::deps::Directory as TargetDependenciesDirectory;
use crate::project::target::Directory as TargetDirectory;

///
/// The Zargo package manager `proof-check` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(
    about = "Runs the full project building, running, trusted setup, proving & verifying sequence"
)]
pub struct Command {
    /// Prints more logs, if passed several times.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbosity: usize,

    /// The path to the Zinc project manifest file.
    #[structopt(
        long = "manifest-path",
        parse(from_os_str),
        default_value = "./Zargo.toml"
    )]
    pub manifest_path: PathBuf,

    /// The contract method to execute. Only for contracts.
    #[structopt(long = "method")]
    pub method: Option<String>,

    /// Uses the release build.
    #[structopt(long = "release")]
    pub is_release: bool,
}

impl Command {
    ///
    /// Executes the command.
    ///
    #[allow(dead_code)]
    pub fn execute(self) -> anyhow::Result<()> {
        let manifest = zinc_manifest::Manifest::try_from(&self.manifest_path)?;

        match manifest.project.r#type {
            zinc_manifest::ProjectType::Contract if self.method.is_none() => {
                anyhow::bail!(Error::MethodMissing)
            }
            _ => {}
        }

        let mut manifest_path = self.manifest_path.clone();
        if manifest_path.is_file() {
            manifest_path.pop();
        }

        DataDirectory::create(&manifest_path)?;
        let data_directory_path = DataDirectory::path(&manifest_path);
        let mut input_path = data_directory_path.clone();
        input_path.push(format!(
            "{}.{}",
            zinc_const::file_name::INPUT,
            zinc_const::extension::JSON,
        ));
        let mut output_path = data_directory_path.clone();
        output_path.push(format!(
            "{}.{}",
            zinc_const::file_name::OUTPUT,
            zinc_const::extension::JSON,
        ));
        if self.method.is_some() && !PrivateKeyFile::exists_at(&data_directory_path) {
            PrivateKeyFile::default().write_to(&data_directory_path)?;
        }
        let mut proving_key_path = data_directory_path.clone();
        proving_key_path.push(zinc_const::file_name::PROVING_KEY);
        let mut verifying_key_path = data_directory_path;
        verifying_key_path.push(zinc_const::file_name::VERIFYING_KEY.to_owned());

        TargetDirectory::create(&manifest_path, self.is_release)?;
        let target_directory_path = TargetDirectory::path(&manifest_path, self.is_release);
        let mut binary_path = target_directory_path;
        binary_path.push(format!(
            "{}.{}",
            zinc_const::file_name::BINARY,
            zinc_const::extension::BINARY
        ));
        TargetDependenciesDirectory::create(&manifest_path)?;

        if self.is_release {
            Compiler::build_release(
                self.verbosity,
                manifest.project.name.as_str(),
                &manifest.project.version,
                &manifest_path,
                false,
            )?;
        } else {
            Compiler::build_debug(
                self.verbosity,
                manifest.project.name.as_str(),
                &manifest.project.version,
                &manifest_path,
                false,
            )?;
        }

        match self.method {
            Some(method) => {
                VirtualMachine::run_contract(
                    self.verbosity,
                    &binary_path,
                    &input_path,
                    &output_path,
                    method.as_str(),
                )?;

                VirtualMachine::setup_contract(
                    self.verbosity,
                    &binary_path,
                    method.as_str(),
                    &proving_key_path,
                    &verifying_key_path,
                )?;

                VirtualMachine::prove_and_verify_contract(
                    self.verbosity,
                    &binary_path,
                    &input_path,
                    &output_path,
                    method.as_str(),
                    &proving_key_path,
                    &verifying_key_path,
                )?;
            }
            None => {
                VirtualMachine::run_circuit(
                    self.verbosity,
                    &binary_path,
                    &input_path,
                    &output_path,
                )?;

                VirtualMachine::setup_circuit(
                    self.verbosity,
                    &binary_path,
                    &proving_key_path,
                    &verifying_key_path,
                )?;

                VirtualMachine::prove_and_verify_circuit(
                    self.verbosity,
                    &binary_path,
                    &input_path,
                    &output_path,
                    &proving_key_path,
                    &verifying_key_path,
                )?;
            }
        }

        Ok(())
    }
}
