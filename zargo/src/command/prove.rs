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
    #[structopt(short = "q", long = "quiet", help = "No output printed to stdout")]
    quiet: bool,

    #[structopt(short = "v", long = "verbose", help = "Use verbose output")]
    verbose: bool,

    #[structopt(
        long = "circuit",
        help = "Path to the circuit binary file",
        default_value = "./build/default.znb"
    )]
    circuit: PathBuf,

    #[structopt(
        long = "params",
        help = "Path to the prover parameters file",
        default_value = "./build/params"
    )]
    params: PathBuf,

    #[structopt(
        long = "input",
        help = "Path to the input JSON file",
        default_value = "./build/input.json"
    )]
    input: PathBuf,

    #[structopt(
        long = "proof",
        help = "Path to the proof file to generate",
        default_value = "./build/proof"
    )]
    proof: PathBuf,
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
                .arg("prove")
                .arg("--circuit")
                .arg(self.circuit)
                .arg("--params")
                .arg(self.params)
                .arg("--input")
                .arg(self.input)
                .arg("--proof")
                .arg(self.proof)
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
