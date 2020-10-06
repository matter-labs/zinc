//!
//! The Zargo package manager `test` subcommand.
//!

use std::convert::TryFrom;
use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

use crate::arguments::command::IExecutable;
use crate::error::directory::Error as DirectoryError;
use crate::error::file::Error as FileError;
use crate::executable::compiler::Compiler;
use crate::executable::compiler::Error as CompilerError;
use crate::executable::virtual_machine::Error as VirtualMachineError;
use crate::executable::virtual_machine::VirtualMachine;
use crate::project::build::Directory as BuildDirectory;
use crate::project::data::Directory as DataDirectory;
use crate::project::manifest::Manifest as ManifestFile;
use crate::project::source::Directory as SourceDirectory;

///
/// The Zargo package manager `test` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Runs the project unit tests")]
pub struct Command {
    /// Prints more logs, if passed several times.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbosity: usize,

    /// The path to the Zargo project manifest file.
    #[structopt(
        long = "manifest-path",
        parse(from_os_str),
        default_value = zinc_const::path::MANIFEST,
    )]
    pub manifest_path: PathBuf,
}

///
/// The Zargo package manager `test` subcommand error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The manifest file error.
    #[fail(display = "manifest file {}", _0)]
    ManifestFile(FileError<toml::de::Error>),
    /// The project binary build directory error.
    #[fail(display = "build directory {}", _0)]
    BuildDirectory(DirectoryError),
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
        let manifest = ManifestFile::try_from(&self.manifest_path).map_err(Error::ManifestFile)?;

        let mut manifest_path = self.manifest_path.clone();
        if manifest_path.is_file() {
            manifest_path.pop();
        }

        let source_directory_path = SourceDirectory::path(&manifest_path);

        let data_directory_path = DataDirectory::path(&manifest_path);

        BuildDirectory::create(&manifest_path).map_err(Error::BuildDirectory)?;
        let build_directory_path = BuildDirectory::path(&manifest_path);
        let mut binary_path = build_directory_path;
        binary_path.push(format!(
            "{}.{}",
            zinc_const::file_name::BINARY,
            zinc_const::extension::BINARY
        ));

        Compiler::build_release(
            self.verbosity,
            manifest.project.name.as_str(),
            manifest.project.version.as_str(),
            &data_directory_path,
            &source_directory_path,
            &binary_path,
            true,
        )
        .map_err(Error::Compiler)?;

        VirtualMachine::test(self.verbosity, &binary_path).map_err(Error::VirtualMachine)?;

        Ok(())
    }
}
