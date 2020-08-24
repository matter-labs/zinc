//!
//! The Zargo project manager `verify` subcommand.
//!

use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

use crate::arguments::command::IExecutable;
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

    /// The path to the binary bytecode file.
    #[structopt(
        long = "binary",
        parse(from_os_str),
        help = "Path to the bytecode file",
        default_value = zinc_const::path::BINARY,
    )]
    pub binary_path: PathBuf,

    /// The path to the public data JSON file.
    #[structopt(
        long = "public-data",
        parse(from_os_str),
        help = "Path to the public data JSON file",
        default_value = zinc_const::path::PUBLIC_DATA,
    )]
    pub public_data_path: PathBuf,

    /// The path to the verifying key file.
    #[structopt(
        long = "verifying-key",
        parse(from_os_str),
        help = "Path to the verifying key file",
        default_value = zinc_const::path::VERIFYING_KEY,
    )]
    pub verifying_key_path: PathBuf,
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
        VirtualMachine::verify(
            self.verbosity,
            &self.binary_path,
            &self.verifying_key_path,
            &self.public_data_path,
        )
        .map_err(Error::VirtualMachine)?;

        Ok(())
    }
}
