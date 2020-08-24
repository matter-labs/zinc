//!
//! The Zargo project manager `proof-check` subcommand.
//!

use std::convert::TryFrom;
use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

use crate::arguments::command::IExecutable;
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
/// The Zargo project manager `proof-check` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(
    about = "Runs the full project building, running, trusted setup, proving & verifying sequence"
)]
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

    /// The path to the witness JSON file.
    #[structopt(
        long = "witness",
        parse(from_os_str),
        help = "Path to the witness JSON file",
        default_value = zinc_const::path::WITNESS,
    )]
    pub witness_path: PathBuf,

    /// The path to the public data JSON file.
    #[structopt(
        long = "public-data",
        parse(from_os_str),
        help = "Path to the public data JSON file",
        default_value = zinc_const::path::PUBLIC_DATA,
    )]
    pub public_data_path: PathBuf,

    /// The path to the proving key file.
    #[structopt(
        long = "proving-key",
        parse(from_os_str),
        help = "Path to the proving key file",
        default_value = zinc_const::path::PROVING_KEY,
    )]
    pub proving_key_path: PathBuf,

    /// The path to the verifying key file.
    #[structopt(
        long = "verifying-key",
        parse(from_os_str),
        help = "Path to the verifying key file",
        default_value = zinc_const::path::VERIFYING_KEY,
    )]
    pub verifying_key_path: PathBuf,

    /// Whether to run the release build.
    #[structopt(long = "release", help = "Run the release build")]
    pub is_release: bool,
}

///
/// The Zargo project manager `proof-check` subcommand error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The manifest file error.
    #[fail(display = "manifest file {}", _0)]
    ManifestFile(ManifestError),
    /// The project binary build directory error.
    #[fail(display = "build directory {}", _0)]
    BuildDirectory(BuildDirectoryError),
    /// The project template, keys, and other auxiliary data directory error.
    #[fail(display = "data directory {}", _0)]
    DataDirectory(DataDirectoryError),
    /// The compiler process error.
    #[fail(display = "compiler {}", _0)]
    Compiler(CompilerError),
    /// The virtual machine `run` process error.
    #[fail(display = "virtual machine 'run' {}", _0)]
    VirtualMachineRun(VirtualMachineError),
    /// The virtual machine `setup` process error.
    #[fail(display = "virtual machine 'setup' {}", _0)]
    VirtualMachineSetup(VirtualMachineError),
    /// The virtual machine `proof-check` process error.
    #[fail(display = "virtual machine 'prove & verify' {}", _0)]
    VirtualMachineProveAndVerify(VirtualMachineError),
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
        DataDirectory::create(&manifest_path).map_err(Error::DataDirectory)?;

        if self.is_release {
            Compiler::build_release(
                self.verbosity,
                manifest.project.name,
                &data_directory_path,
                &source_directory_path,
                &self.binary_path,
                false,
            )
            .map_err(Error::Compiler)?;
        } else {
            Compiler::build_debug(
                self.verbosity,
                manifest.project.name,
                &data_directory_path,
                &source_directory_path,
                &self.binary_path,
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
        .map_err(Error::VirtualMachineRun)?;

        VirtualMachine::setup(
            self.verbosity,
            &self.binary_path,
            &self.proving_key_path,
            &self.verifying_key_path,
        )
        .map_err(Error::VirtualMachineSetup)?;

        VirtualMachine::prove_and_verify(
            self.verbosity,
            &self.binary_path,
            &self.witness_path,
            &self.public_data_path,
            &self.proving_key_path,
            &self.verifying_key_path,
        )
        .map_err(Error::VirtualMachineProveAndVerify)?;

        Ok(())
    }
}
