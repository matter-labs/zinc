//!
//! The Zargo project manager `prove` subcommand.
//!

use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

use crate::arguments::command::IExecutable;
use crate::executable::virtual_machine::Error as VirtualMachineError;
use crate::executable::virtual_machine::VirtualMachine;

///
/// The Zargo project manager `prove` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Generates the zero-knowledge proof for given witness data")]
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

    /// The path to the witness JSON file.
    #[structopt(
        long = "witness",
        parse(from_os_str),
        help = "Path to the witness JSON file",
        default_value = zinc_const::path::WITNESS,
    )]
    pub witness_path: PathBuf,

    /// The path to the public data JSON file.
    #[structopt(
        long = "public-data",
        parse(from_os_str),
        help = "Path to the public data JSON file",
        default_value = zinc_const::path::PUBLIC_DATA,
    )]
    pub public_data_path: PathBuf,

    /// The path to the proving key file.
    #[structopt(
        long = "proving-key",
        parse(from_os_str),
        help = "Path to the proving key file",
        default_value = zinc_const::path::PROVING_KEY,
    )]
    pub proving_key: PathBuf,
}

///
/// The Zargo project manager `prove` subcommand error.
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
        VirtualMachine::prove(
            self.verbosity,
            &self.binary_path,
            &self.proving_key,
            &self.witness_path,
            &self.public_data_path,
        )
        .map_err(Error::VirtualMachine)?;

        Ok(())
    }
}
