//!
//! The `verify` command.
//!

use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

use crate::executable::virtual_machine::Error as VirtualMachineError;
use crate::executable::virtual_machine::VirtualMachine;

#[derive(Debug, StructOpt)]
#[structopt(about = "Verifies the zero-knowledge proof")]
pub struct Command {
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
    verbosity: usize,

    #[structopt(long = "build", help = "Path to the build binary file")]
    binary_path: PathBuf,

    #[structopt(long = "public-data", help = "Path to the public data JSON file")]
    public_data_path: PathBuf,

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
