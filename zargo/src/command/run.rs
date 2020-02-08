//!
//! The `run` command.
//!

use std::convert::TryFrom;
use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

use crate::directory::build::Directory as BuildDirectory;
use crate::directory::build::Error as BuildDirectoryError;
use crate::directory::data::Directory as DataDirectory;
use crate::directory::data::Error as DataDirectoryError;
use crate::directory::source::Directory as SourceDirectory;
use crate::directory::source::Error as SourceDirectoryError;
use crate::executable::compiler::Compiler;
use crate::executable::compiler::Error as CompilerError;
use crate::executable::virtual_machine::Error as VirtualMachineError;
use crate::executable::virtual_machine::VirtualMachine;
use crate::manifest::Error as ManifestError;
use crate::manifest::Manifest;

#[derive(Debug, StructOpt)]
#[structopt(about = "Runs a circuit and saves its output")]
pub struct Command {
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
    verbosity: usize,

    #[structopt(
        long = "manifest-path",
        help = "Path to Zargo.toml",
        default_value = "./Zargo.toml"
    )]
    manifest_path: PathBuf,

    #[structopt(
        long = "circuit",
        help = "Path to the circuit binary file",
        default_value = "./build/default.znb"
    )]
    circuit: PathBuf,

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
    #[fail(display = "manifest file: {}", _0)]
    ManifestFile(ManifestError),
    #[fail(display = "source directory: {}", _0)]
    SourceDirectory(SourceDirectoryError),
    #[fail(display = "build directory: {}", _0)]
    BuildDirectory(BuildDirectoryError),
    #[fail(display = "data directory: {}", _0)]
    DataDirectory(DataDirectoryError),
    #[fail(display = "compiler: {}", _0)]
    Compiler(CompilerError),
    #[fail(display = "virtual machine: {}", _0)]
    VirtualMachine(VirtualMachineError),
}

impl Command {
    pub fn execute(self) -> Result<(), Error> {
        let _manifest = Manifest::try_from(&self.manifest_path).map_err(Error::ManifestFile)?;

        let mut circuit_path = self.manifest_path.clone();
        if circuit_path.is_file() {
            circuit_path.pop();
        }

        let source_file_paths =
            SourceDirectory::files(&circuit_path).map_err(Error::SourceDirectory)?;

        BuildDirectory::create(&circuit_path).map_err(Error::BuildDirectory)?;
        DataDirectory::create(&circuit_path).map_err(Error::DataDirectory)?;

        Compiler::build(
            self.verbosity,
            &self.witness,
            &self.public_data,
            &self.circuit,
            &source_file_paths,
        )
        .map_err(Error::Compiler)?;

        VirtualMachine::run(
            self.verbosity,
            &self.circuit,
            &self.witness,
            &self.public_data,
        )
        .map_err(Error::VirtualMachine)?;

        Ok(())
    }
}
