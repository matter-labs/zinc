//!
//! The Zargo `prove` command.
//!

use std::io;
use std::path::PathBuf;
use std::process;
use std::process::ExitStatus;

use failure::Fail;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Generates the zero-knowledge proof for given witness data")]
pub struct Command {
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
    verbose: usize,

    #[structopt(
        long = "circuit",
        help = "Path to the circuit binary file",
        default_value = "./build/default.znb"
    )]
    circuit: PathBuf,

    #[structopt(
        long = "proving-key",
        help = "Path to the proving key file",
        default_value = "./build/proving-key"
    )]
    proving_key: PathBuf,

    #[structopt(
        long = "witness",
        help = "Path to the witness JSON file",
        default_value = "./build/witness.json"
    )]
    witness: PathBuf,

    #[structopt(
        long = "pubdata",
        help = "Path to the pubdata JSON file to write",
        default_value = "./build/public-data.json"
    )]
    pubdata: PathBuf,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "virtual machine process spawning: {}", _0)]
    VirtualMachineProcessSpawning(io::Error),
    #[fail(display = "virtual machine process waiting: {}", _0)]
    VirtualMachineProcessWaiting(io::Error),
    #[fail(display = "virtual machine process failure: {}", _0)]
    VirtualMachineProcessFailure(ExitStatus),
}

impl Command {
    pub fn execute(self) -> Result<(), Error> {
        let mut virtual_machine_process =
            process::Command::new(crate::constants::ZINC_VIRTUAL_MACHINE_BINARY_NAME)
                .args(vec!["-v"; self.verbose])
                .arg("prove")
                .arg("--circuit")
                .arg(self.circuit)
                .arg("--proving-key")
                .arg(self.proving_key)
                .arg("--witness")
                .arg(self.witness)
                .arg("--pubdata")
                .arg(self.pubdata)
                .spawn()
                .map_err(Error::VirtualMachineProcessSpawning)?;
        let virtual_machine_process_status = virtual_machine_process
            .wait()
            .map_err(Error::VirtualMachineProcessWaiting)?;
        if !virtual_machine_process_status.success() {
            return Err(Error::VirtualMachineProcessFailure(
                virtual_machine_process_status,
            ));
        }

        Ok(())
    }
}
