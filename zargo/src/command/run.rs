//!
//! The Zargo `exec` command.
//!

use std::io;
use std::path::PathBuf;
use std::process;
use std::process::ExitStatus;

use failure::Fail;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Runs a circuit and saves its output")]
pub struct Command {
    #[structopt(short = "q", long = "quiet", help = "No output printed to stdout")]
    quiet: bool,

    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
    pub verbose: usize,

    #[structopt(
        long = "circuit",
        help = "Path to the circuit binary file",
        default_value = "./build/default.znb"
    )]
    circuit: PathBuf,

    #[structopt(
        long = "input",
        help = "Path to the input witness JSON file",
        default_value = "./build/witness.json"
    )]
    input: PathBuf,

    #[structopt(
        long = "output",
        help = "Path to the output public data JSON file",
        default_value = "./build/public-data.json"
    )]
    output: PathBuf,
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
                .arg("run")
                .arg("--circuit")
                .arg(self.circuit)
                .arg("--input")
                .arg(self.input)
                .arg("--output")
                .arg(self.output)
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
