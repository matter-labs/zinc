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
#[structopt(about = "Runs the project and saves its output")]
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
        long = "build",
        help = "Path to the build directory",
        default_value = "./build/"
    )]
    build_path: PathBuf,

    #[structopt(
        long = "data",
        help = "Path to the data directory",
        default_value = "./data/"
    )]
    data_path: PathBuf,

    #[structopt(
        long = "entry",
        help = "The bytecode entry name",
        default_value = "main"
    )]
    entry_name: String,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "manifest file {}", _0)]
    ManifestFile(ManifestError),
    #[fail(display = "source directory {}", _0)]
    SourceDirectory(SourceDirectoryError),
    #[fail(display = "build directory {}", _0)]
    BuildDirectory(BuildDirectoryError),
    #[fail(display = "data directory {}", _0)]
    DataDirectory(DataDirectoryError),
    #[fail(display = "compiler {}", _0)]
    Compiler(CompilerError),
    #[fail(display = "virtual machine {}", _0)]
    VirtualMachine(VirtualMachineError),
}

impl Command {
    pub fn execute(mut self) -> Result<(), Error> {
        let _manifest = Manifest::try_from(&self.manifest_path).map_err(Error::ManifestFile)?;

        let mut manifest_path = self.manifest_path.clone();
        if manifest_path.is_file() {
            manifest_path.pop();
        }

        let source_file_paths =
            SourceDirectory::files(&manifest_path).map_err(Error::SourceDirectory)?;

        BuildDirectory::create(&manifest_path).map_err(Error::BuildDirectory)?;
        DataDirectory::create(&manifest_path).map_err(Error::DataDirectory)?;

        Compiler::build(
            self.verbosity,
            &self.data_path,
            &self.build_path,
            &source_file_paths,
        )
        .map_err(Error::Compiler)?;

        self.build_path.push(format!("{}.znb", self.entry_name));

        let mut witness_path = self.data_path.clone();
        witness_path.push(format!("{}_witness.json", self.entry_name));

        let mut public_data_path = self.data_path.clone();
        public_data_path.push(format!("{}_public_data.json", self.entry_name));

        VirtualMachine::run(
            self.verbosity,
            &self.build_path,
            &witness_path,
            &public_data_path,
        )
        .map_err(Error::VirtualMachine)?;

        Ok(())
    }
}
