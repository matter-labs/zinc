//!
//! The Zargo project manager `setup` subcommand.
//!

use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

use crate::arguments::command::IExecutable;
use crate::executable::virtual_machine::Error as VirtualMachineError;
use crate::executable::virtual_machine::VirtualMachine;

///
/// The Zargo project manager `setup` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Generates a pair of the proving and verifying keys")]
pub struct Command {
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
    verbosity: usize,

    #[structopt(
        long = "binary",
        help = "Path to the binary data file",
        default_value = "./build/main.znb"
    )]
    binary_path: PathBuf,

    #[structopt(
        long = "proving-key",
        help = "Path to the proving key file",
        default_value = "./data/proving_key"
    )]
    proving_key_path: PathBuf,

    #[structopt(
        long = "verifying-key",
        help = "Path to the verifying key file",
        default_value = "./data/verifying_key.txt"
    )]
    verifying_key_path: PathBuf,
}

///
/// The Zargo project manager `setup` subcommand error.
///
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "virtual machine {}", _0)]
    VirtualMachine(VirtualMachineError),
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(self) -> Result<(), Self::Error> {
        VirtualMachine::setup(
            self.verbosity,
            &self.binary_path,
            &self.proving_key_path,
            &self.verifying_key_path,
        )
        .map_err(Error::VirtualMachine)?;

        Ok(())
    }
}
