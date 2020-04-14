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

    #[structopt(
        long = "circuit",
        help = "Path to the circuit binary file",
        default_value = "./build/default.znb"
    )]
    circuit: PathBuf,

    #[structopt(
        long = "verifying-key",
        help = "Path to the verifying key file",
        default_value = "./data/verifying-key.txt"
    )]
    verifying_key: PathBuf,

    #[structopt(
        long = "public-data",
        help = "Path to the program's public data JSON file",
        default_value = "./data/public-data.json"
    )]
    public_data: PathBuf,
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
            &self.circuit,
            &self.verifying_key,
            &self.public_data,
        )
        .map_err(Error::VirtualMachine)?;

        Ok(())
    }
}
