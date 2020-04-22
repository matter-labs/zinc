//!
//! The `prove` command.
//!

use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

use crate::executable::virtual_machine::Error as VirtualMachineError;
use crate::executable::virtual_machine::VirtualMachine;

#[derive(Debug, StructOpt)]
#[structopt(about = "Generates the zero-knowledge proof for given witness data")]
pub struct Command {
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
    verbosity: usize,

    #[structopt(
        long = "binary",
        help = "Path to the binary file",
        default_value = "./build/default.znb"
    )]
    binary: PathBuf,

    #[structopt(
        long = "proving-key",
        help = "Path to the proving key file",
        default_value = "./data/proving-key"
    )]
    proving_key: PathBuf,

    #[structopt(
        long = "witness",
        help = "Path to the witness JSON file",
        default_value = "./data/witness.json"
    )]
    witness: PathBuf,

    #[structopt(
        long = "public-data",
        help = "Path to the public data JSON file to write",
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
        VirtualMachine::prove(
            self.verbosity,
            &self.binary,
            &self.proving_key,
            &self.witness,
            &self.public_data,
        )
        .map_err(Error::VirtualMachine)?;

        Ok(())
    }
}
