//!
//! The Zargo package manager `setup` subcommand.
//!

pub mod error;

use std::convert::TryFrom;
use std::path::PathBuf;

use structopt::StructOpt;

use zinc_manifest::Manifest;
use zinc_manifest::ProjectType;

use crate::executable::virtual_machine::VirtualMachine;
use crate::project::build::Directory as BuildDirectory;
use crate::project::data::Directory as DataDirectory;

use self::error::Error;

///
/// The Zargo package manager `setup` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Generates a pair of proving and verifying keys")]
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

    /// The contract method to do the setup for. Only for contracts.
    #[structopt(long = "method")]
    pub method: Option<String>,
}

impl Command {
    ///
    /// Executes the command.
    ///
    pub fn execute(self) -> Result<(), Error> {
        let manifest = Manifest::try_from(&self.manifest_path).map_err(Error::Manifest)?;

        match manifest.project.r#type {
            ProjectType::Contract if self.method.is_none() => return Err(Error::MethodMissing),
            _ => {}
        }

        let mut manifest_path = self.manifest_path.clone();
        if manifest_path.is_file() {
            manifest_path.pop();
        }

        let data_directory_path = DataDirectory::path(&manifest_path);
        let mut proving_key_path = data_directory_path.clone();
        proving_key_path.push(zinc_const::file_name::PROVING_KEY);
        let mut verifying_key_path = data_directory_path;
        verifying_key_path.push(zinc_const::file_name::VERIFYING_KEY.to_owned());

        let build_directory_path = BuildDirectory::path(&manifest_path);
        let mut binary_path = build_directory_path;
        binary_path.push(format!(
            "{}.{}",
            zinc_const::file_name::BINARY,
            zinc_const::extension::BINARY
        ));

        match self.method {
            Some(method) => VirtualMachine::setup_contract(
                self.verbosity,
                &binary_path,
                method.as_str(),
                &proving_key_path,
                &verifying_key_path,
            ),
            None => VirtualMachine::setup_circuit(
                self.verbosity,
                &binary_path,
                &proving_key_path,
                &verifying_key_path,
            ),
        }
        .map_err(Error::VirtualMachine)?;

        Ok(())
    }
}
