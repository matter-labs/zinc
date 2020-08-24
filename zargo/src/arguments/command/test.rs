//!
//! The Zargo project manager `test` subcommand.
//!

use std::convert::TryFrom;
use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

use crate::arguments::command::IExecutable;
use crate::directory::build::Directory as BuildDirectory;
use crate::directory::build::Error as BuildDirectoryError;
use crate::directory::data::Directory as DataDirectory;
use crate::directory::source::Directory as SourceDirectory;
use crate::executable::compiler::Compiler;
use crate::executable::compiler::Error as CompilerError;
use crate::executable::virtual_machine::Error as VirtualMachineError;
use crate::executable::virtual_machine::VirtualMachine;
use crate::manifest::Error as ManifestError;
use crate::manifest::Manifest;

///
/// The Zargo project manager `test` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Runs the project and saves its output")]
pub struct Command {
    /// The logging level value, which helps the logger to set the logging level.
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
    pub verbosity: usize,

    /// The path to the Zargo project manifest file.
    #[structopt(
        long = "manifest-path",
        parse(from_os_str),
        help = "Path to Zargo.toml",
        default_value = zinc_const::path::MANIFEST,
    )]
    pub manifest_path: PathBuf,

    /// The path to the binary bytecode file.
    #[structopt(
        long = "binary",
        parse(from_os_str),
        help = "Path to the bytecode file",
        default_value = zinc_const::path::BINARY,
    )]
    pub binary_path: PathBuf,
}

///
/// The Zargo project manager `test` subcommand error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The manifest file error.
    #[fail(display = "manifest file {}", _0)]
    ManifestFile(ManifestError),
    /// The project binary build directory error.
    #[fail(display = "build directory {}", _0)]
    BuildDirectory(BuildDirectoryError),
    /// The compiler process error.
    #[fail(display = "compiler {}", _0)]
    Compiler(CompilerError),
    /// The virtual machine process error.
    #[fail(display = "virtual machine {}", _0)]
    VirtualMachine(VirtualMachineError),
}

///
/// The unit test summary.
///
#[derive(Default)]
pub struct Summary {
    pub passed: u8,
    pub failed: u8,
    pub invalid: u8,
    pub ignored: u8,
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(self) -> Result<(), Self::Error> {
        let manifest = Manifest::try_from(&self.manifest_path).map_err(Error::ManifestFile)?;

        let mut manifest_path = self.manifest_path.clone();
        if manifest_path.is_file() {
            manifest_path.pop();
        }

        let data_directory_path = DataDirectory::path(&manifest_path);
        let source_directory_path = SourceDirectory::path(&manifest_path);

        BuildDirectory::create(&manifest_path).map_err(Error::BuildDirectory)?;

        Compiler::build_release(
            self.verbosity,
            manifest.project.name,
            &data_directory_path,
            &source_directory_path,
            &self.binary_path,
            true,
        )
        .map_err(Error::Compiler)?;

        VirtualMachine::test(self.verbosity, &self.binary_path).map_err(Error::VirtualMachine)?;

        Ok(())
    }
}
