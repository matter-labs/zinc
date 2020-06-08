//!
//! The `setup` command.
//!

use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

use crate::executable::virtual_machine::Error as VirtualMachineError;
use crate::executable::virtual_machine::VirtualMachine;

#[derive(Debug, StructOpt)]
#[structopt(about = "Generates a pair of the proving and verifying keys")]
pub struct Command {
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
    verbosity: usize,

    #[structopt(long = "binary", help = "Path to the binary data file")]
    binary_path: PathBuf,

    #[structopt(long = "proving-key", help = "Path to the proving key file")]
    proving_key_path: PathBuf,

    #[structopt(long = "verifying-key", help = "Path to the verifying key file")]
    verifying_key_path: PathBuf,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "virtual machine {}", _0)]
    VirtualMachine(VirtualMachineError),
}

impl Command {
    pub fn execute(self) -> Result<(), Error> {
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
