//!
//! The Zargo project manager `run` subcommand.
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
use crate::file::error::Error as FileError;
use crate::file::manifest::project_type::ProjectType;
use crate::file::manifest::Manifest as ManifestFile;

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
        parse(from_os_str),
        help = "Path to Zargo.toml",
        default_value = zinc_const::path::MANIFEST,
    )]
    pub manifest_path: PathBuf,

    /// The contract method to call. Only for contracts.
    #[structopt(long = "method", help = "The contract method to call")]
    pub method: Option<String>,

    /// Whether to run the release version.
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
    ManifestFile(FileError<toml::de::Error>),
    /// The contract method to call is missing.
    #[fail(display = "contract method to call must be specified")]
    MethodMissing,
    /// The project binary build directory error.
    #[fail(display = "build directory {}", _0)]
    BuildDirectory(BuildDirectoryError),
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
        let manifest = ManifestFile::try_from(&self.manifest_path).map_err(Error::ManifestFile)?;

        match manifest.project.r#type {
            ProjectType::Contract if self.method.is_none() => return Err(Error::MethodMissing),
            _ => {}
        }

        let mut manifest_path = self.manifest_path.clone();
        if manifest_path.is_file() {
            manifest_path.pop();
        }

        let source_directory_path = SourceDirectory::path(&manifest_path);

        DataDirectory::create(&manifest_path).map_err(Error::DataDirectory)?;
        let data_directory_path = DataDirectory::path(&manifest_path);
        let mut witness_path = data_directory_path.clone();
        let mut public_data_path = data_directory_path.clone();
        if let Some(ref method) = self.method {
            witness_path.push(format!(
                "{}_{}.{}",
                zinc_const::file_name::WITNESS,
                method,
                zinc_const::extension::JSON,
            ));
            public_data_path.push(format!(
                "{}_{}.{}",
                zinc_const::file_name::PUBLIC_DATA,
                method,
                zinc_const::extension::JSON,
            ));
        } else {
            witness_path.push(format!(
                "{}.{}",
                zinc_const::file_name::WITNESS,
                zinc_const::extension::JSON,
            ));
            public_data_path.push(format!(
                "{}.{}",
                zinc_const::file_name::PUBLIC_DATA,
                zinc_const::extension::JSON,
            ));
        }
        let mut storage_path = data_directory_path.clone();
        storage_path.push(format!(
            "{}.{}",
            zinc_const::file_name::STORAGE,
            zinc_const::extension::JSON
        ));

        BuildDirectory::create(&manifest_path).map_err(Error::BuildDirectory)?;
        let build_directory_path = BuildDirectory::path(&manifest_path);
        let mut binary_path = build_directory_path;
        binary_path.push(format!(
            "{}.{}",
            zinc_const::file_name::BINARY,
            zinc_const::extension::BINARY
        ));

        if self.is_release {
            Compiler::build_release(
                self.verbosity,
                manifest.project.name.as_str(),
                manifest.project.version.as_str(),
                &data_directory_path,
                &source_directory_path,
                &binary_path,
                false,
            )
            .map_err(Error::Compiler)?;
        } else {
            Compiler::build_debug(
                self.verbosity,
                manifest.project.name.as_str(),
                manifest.project.version.as_str(),
                &data_directory_path,
                &source_directory_path,
                &binary_path,
                false,
            )
            .map_err(Error::Compiler)?;
        }

        match self.method {
            Some(method) => VirtualMachine::run_contract(
                self.verbosity,
                &binary_path,
                &witness_path,
                &public_data_path,
                &storage_path,
                method.as_str(),
            ),
            None => VirtualMachine::run_circuit(
                self.verbosity,
                &binary_path,
                &witness_path,
                &public_data_path,
            ),
        }
        .map_err(Error::VirtualMachine)?;

        Ok(())
    }
}
