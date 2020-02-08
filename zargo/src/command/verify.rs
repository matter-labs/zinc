//!
//! The Zargo `verify` command.
//!

use std::io;
use std::path::PathBuf;
use std::process;
use std::process::ExitStatus;

use failure::Fail;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Verifies the zero-knowledge proof")]
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
        short = "k",
        long = "verifying-key",
        help = "Path to the verifying key file",
        default_value = "./build/verifying-key.txt"
    )]
    verifying_key: PathBuf,

    #[structopt(
        long = "public-data",
        help = "Path to the program's public data JSON file",
        default_value = "./build/public-data.json"
    )]
    public_data: PathBuf,
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
            process::Command::new(crate::constants::ZINC_BINARY_NAME_VIRTUAL_MACHINE)
                .args(vec!["-v"; self.verbose])
                .arg("verify")
                .arg("--circuit")
                .arg(self.circuit)
                .arg("--verifying-key")
                .arg(self.verifying_key)
                .arg("--public-data")
                .arg(self.public_data)
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
