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

    #[structopt(
        long = "circuit",
        help = "Path to the circuit binary file",
        default_value = "./build/default.znb"
    )]
    circuit: PathBuf,

    #[structopt(
        long = "proving-key",
        help = "Path to the proving key file to generate",
        default_value = "./data/proving-key"
    )]
    proving_key: PathBuf,

    #[structopt(
        long = "verifying-key",
        help = "Path to the verifying key file to generate",
        default_value = "./data/verifying-key.txt"
    )]
    verifying_key: PathBuf,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "virtual machine: {}", _0)]
    VirtualMachine(VirtualMachineError),
}

impl Command {
    pub fn execute(self) -> Result<(), Error> {
        VirtualMachine::setup(
            self.verbosity,
            &self.circuit,
            &self.proving_key,
            &self.verifying_key,
        )
        .map_err(Error::VirtualMachine)?;

        Ok(())
    }
}
