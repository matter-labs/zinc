//!
//! The Zargo project manager `verify` subcommand.
//!

use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

use crate::arguments::command::IExecutable;
use crate::directory::build::Directory as BuildDirectory;
use crate::directory::data::Directory as DataDirectory;
use crate::executable::virtual_machine::Error as VirtualMachineError;
use crate::executable::virtual_machine::VirtualMachine;

///
/// The Zargo project manager `verify` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Verifies the zero-knowledge proof")]
pub struct Command {
    /// The logging level value, which helps the logger to set the logging level.
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
    pub verbosity: usize,

    /// The path to the Zargo project manifest file.
    #[structopt(
        long = "manifest-path",
        help = "Path to Zargo.toml",
        default_value = zinc_const::path::MANIFEST,
    )]
    pub manifest_path: PathBuf,

    /// The contract method to call. Only for contracts.
    #[structopt(long = "method", help = "The contract method to call")]
    pub method: Option<String>,
}

///
/// The Zargo project manager `verify` subcommand error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The virtual machine process error.
    #[fail(display = "virtual machine {}", _0)]
    VirtualMachine(VirtualMachineError),
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(self) -> Result<(), Self::Error> {
        let mut manifest_path = self.manifest_path.clone();
        if manifest_path.is_file() {
            manifest_path.pop();
        }

        let data_directory_path = DataDirectory::path(&manifest_path);
        let mut public_data_path = data_directory_path.clone();
        if let Some(ref method) = self.method {
            public_data_path.push(format!(
                "{}_{}.{}",
                zinc_const::file_name::PUBLIC_DATA,
                method,
                zinc_const::extension::JSON,
            ));
        } else {
            public_data_path.push(format!(
                "{}.{}",
                zinc_const::file_name::PUBLIC_DATA,
                zinc_const::extension::JSON,
            ));
        }
        let mut verifying_key_path = data_directory_path;
        verifying_key_path.push(format!(
            "{}.{}",
            zinc_const::file_name::VERIFYING_KEY,
            zinc_const::extension::VERIFYING_KEY
        ));

        let build_directory_path = BuildDirectory::path(&manifest_path);
        let mut binary_path = build_directory_path;
        binary_path.push(format!(
            "{}.{}",
            zinc_const::file_name::BINARY,
            zinc_const::extension::BINARY
        ));

        match self.method {
            Some(method) => VirtualMachine::verify_contract(
                self.verbosity,
                &binary_path,
                &verifying_key_path,
                &public_data_path,
                method.as_str(),
            ),
            _ => VirtualMachine::verify_circuit(
                self.verbosity,
                &binary_path,
                &verifying_key_path,
                &public_data_path,
            ),
        }
        .map_err(Error::VirtualMachine)?;

        Ok(())
    }
}
