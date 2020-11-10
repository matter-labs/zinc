//!
//! The Zargo package manager `run` subcommand.
//!

use std::convert::TryFrom;
use std::path::PathBuf;

use structopt::StructOpt;

use zinc_manifest::Manifest;
use zinc_manifest::ProjectType;

use crate::error::Error;
use crate::executable::compiler::Compiler;
use crate::executable::virtual_machine::VirtualMachine;
use crate::project::build::Directory as BuildDirectory;
use crate::project::data::private_key::PrivateKey as PrivateKeyFile;
use crate::project::data::Directory as DataDirectory;
use crate::project::source::Directory as SourceDirectory;

///
/// The Zargo package manager `run` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Runs the project and prints its output")]
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

    /// The contract method to call. Only for contracts.
    #[structopt(long = "method")]
    pub method: Option<String>,

    /// Runs the release build.
    #[structopt(long = "release")]
    pub is_release: bool,
}

impl Command {
    ///
    /// Executes the command.
    ///
    pub fn execute(self) -> anyhow::Result<()> {
        let manifest = Manifest::try_from(&self.manifest_path)?;

        match manifest.project.r#type {
            ProjectType::Contract if self.method.is_none() => anyhow::bail!(Error::MethodMissing),
            _ => {}
        }

        let mut manifest_path = self.manifest_path.clone();
        if manifest_path.is_file() {
            manifest_path.pop();
        }

        let source_directory_path = SourceDirectory::path(&manifest_path);

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

        BuildDirectory::create(&manifest_path)?;
        let build_directory_path = BuildDirectory::path(&manifest_path);
        let mut binary_path = build_directory_path;
        binary_path.push(format!(
            "{}.{}",
            zinc_const::file_name::BINARY,
            zinc_const::extension::BINARY
        ));

        if self.is_release {
            Compiler::build_release(
                self.verbosity,
                manifest.project.name.as_str(),
                manifest.project.version.as_str(),
                &manifest_path,
                &data_directory_path,
                &source_directory_path,
                &binary_path,
                false,
            )?;
        } else {
            Compiler::build_debug(
                self.verbosity,
                manifest.project.name.as_str(),
                manifest.project.version.as_str(),
                &manifest_path,
                &data_directory_path,
                &source_directory_path,
                &binary_path,
                false,
            )?;
        }

        match self.method {
            Some(method) => VirtualMachine::run_contract(
                self.verbosity,
                &binary_path,
                &input_path,
                &output_path,
                method.as_str(),
            ),
            None => {
                VirtualMachine::run_circuit(self.verbosity, &binary_path, &input_path, &output_path)
            }
        }?;

        Ok(())
    }
}
