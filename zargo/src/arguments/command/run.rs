//!
//! The Zargo project manager `run` subcommand.
//!

use std::convert::TryFrom;
use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

use crate::arguments::command::IExecutable;
use crate::directory::build::test::Directory as TestBuildDirectory;
use crate::directory::build::test::Error as TestBuildDirectoryError;
use crate::directory::build::Directory as BuildDirectory;
use crate::directory::build::Error as BuildDirectoryError;
use crate::directory::data::Directory as DataDirectory;
use crate::directory::data::Error as DataDirectoryError;
use crate::directory::source::Directory as SourceDirectory;
use crate::executable::compiler::Compiler;
use crate::executable::compiler::Error as CompilerError;
use crate::executable::virtual_machine::Error as VirtualMachineError;
use crate::executable::virtual_machine::VirtualMachine;
use crate::manifest::Error as ManifestError;
use crate::manifest::Manifest;

///
/// The Zargo project manager `run` subcommand.
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
        help = "Path to Zargo.toml",
        default_value = "./Zargo.toml"
    )]
    pub manifest_path: PathBuf,

    /// The path to the binary bytecode file.
    #[structopt(
        long = "binary",
        help = "Path to the bytecode file",
        default_value = "./build/main.znb"
    )]
    pub binary_path: PathBuf,

    /// The path to the witness JSON file.
    #[structopt(
        long = "witness",
        help = "Path to the witness JSON file",
        default_value = "./data/main_witness.json"
    )]
    pub witness_path: PathBuf,

    /// The path to the public data JSON file.
    #[structopt(
        long = "public-data",
        help = "Path to the public data JSON file",
        default_value = "./data/main_public_data.json"
    )]
    pub public_data_path: PathBuf,

    /// Whether to run the release build.
    #[structopt(long = "release", help = "Run the release build")]
    pub is_release: bool,
}

///
/// The Zargo project manager `run` subcommand error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The manifest file error.
    #[fail(display = "manifest file {}", _0)]
    ManifestFile(ManifestError),
    /// The project binary build directory error.
    #[fail(display = "build directory {}", _0)]
    BuildDirectory(BuildDirectoryError),
    /// The project unit tests binary build directory error.
    #[fail(display = "test build directory {}", _0)]
    TestBuildDirectory(TestBuildDirectoryError),
    /// The project template, keys, and other auxiliary data directory error.
    #[fail(display = "data directory {}", _0)]
    DataDirectory(DataDirectoryError),
    /// The compiler process error.
    #[fail(display = "compiler {}", _0)]
    Compiler(CompilerError),
    /// The virtual machine process error.
    #[fail(display = "virtual machine {}", _0)]
    VirtualMachine(VirtualMachineError),
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(self) -> Result<(), Self::Error> {
        let _manifest = Manifest::try_from(&self.manifest_path).map_err(Error::ManifestFile)?;

        let mut manifest_path = self.manifest_path.clone();
        if manifest_path.is_file() {
            manifest_path.pop();
        }

        let source_directory_path = SourceDirectory::path(&manifest_path);
        let build_directory_path = BuildDirectory::path(&manifest_path);
        let data_directory_path = DataDirectory::path(&manifest_path);

        BuildDirectory::create(&manifest_path).map_err(Error::BuildDirectory)?;
        TestBuildDirectory::create(&manifest_path).map_err(Error::TestBuildDirectory)?;
        DataDirectory::create(&manifest_path).map_err(Error::DataDirectory)?;

        if self.is_release {
            Compiler::build_release(
                self.verbosity,
                &data_directory_path,
                &build_directory_path,
                &source_directory_path,
                false,
            )
            .map_err(Error::Compiler)?;
        } else {
            Compiler::build_debug(
                self.verbosity,
                &data_directory_path,
                &build_directory_path,
                &source_directory_path,
                false,
            )
            .map_err(Error::Compiler)?;
        }

        VirtualMachine::run(
            self.verbosity,
            &self.binary_path,
            &self.witness_path,
            &self.public_data_path,
        )
        .map_err(Error::VirtualMachine)?;

        Ok(())
    }
}
