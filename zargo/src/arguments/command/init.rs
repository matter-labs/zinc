//!
//! The Zargo project manager `init` subcommand.
//!

use std::ffi::OsString;
use std::path::PathBuf;
use std::str::FromStr;

use failure::Fail;
use structopt::StructOpt;

use crate::arguments::command::IExecutable;
use crate::directory::source::circuit::Circuit as CircuitFile;
use crate::directory::source::circuit::Error as CircuitFileError;
use crate::directory::source::contract::Contract as ContractFile;
use crate::directory::source::contract::Error as ContractFileError;
use crate::directory::source::Directory as SourceDirectory;
use crate::directory::source::Error as SourceDirectoryError;
use crate::manifest::project_type::ProjectType;
use crate::manifest::Error as ManifestError;
use crate::manifest::Manifest;

///
/// The Zargo project manager `init` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Initializes a new project in the specified directory")]
pub struct Command {
    /// The logging level value, which helps the logger to set the logging level.
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
    pub verbosity: usize,

    /// The project name, which will appear in the Zargo manifest file.
    #[structopt(
        long = "name",
        help = "Set the project name, defaults to the directory name"
    )]
    pub name: Option<String>,

    /// The project type, which is either a circuit or contract for now.
    #[structopt(
        long = "type",
        help = "Set the project type, either 'circuit' or 'contract'"
    )]
    pub r#type: String,

    /// The path to the project directory to initialize.
    #[structopt(parse(from_os_str), default_value = "./")]
    pub path: PathBuf,
}

///
/// The Zargo project manager `init` subcommand error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The invalid project name error.
    #[fail(
        display = "project name is missing and cannot be inferred from path {:?}",
        _0
    )]
    ProjectNameInvalid(OsString),
    /// The invalid project type error.
    #[fail(
        display = "project type must be either `circuit` or `contract`, found `{}`",
        _0
    )]
    ProjectTypeInvalid(String),
    /// The project directory does not exist. Use `new` instead.
    #[fail(
        display = "directory {:?} does not exist. To create a new directory, use `zargo new`",
        _0
    )]
    DirectoryDoesNotExist(OsString),
    /// The project has been already initialized error.
    #[fail(display = "project at path {:?} is already initialized", _0)]
    CircuitAlreadyInitialized(OsString),
    /// The manifest file error.
    #[fail(display = "manifest file {}", _0)]
    ManifestFile(ManifestError),
    /// The project source code directory error.
    #[fail(display = "source directory {}", _0)]
    SourceDirectory(SourceDirectoryError),
    /// The circuit source code entry point file generation error.
    #[fail(display = "main file {}", _0)]
    CircuitFile(CircuitFileError),
    /// The contract source code entry point file generation error.
    #[fail(display = "contract file {}", _0)]
    ContractFile(ContractFileError),
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(mut self) -> Result<(), Error> {
        let project_name = match self.name.take() {
            Some(name) => name,
            None => self
                .path
                .file_stem()
                .ok_or_else(|| Error::ProjectNameInvalid(self.path.as_os_str().to_owned()))?
                .to_string_lossy()
                .to_string(),
        };

        let project_type =
            ProjectType::from_str(self.r#type.as_str()).map_err(Error::ProjectTypeInvalid)?;

        if !self.path.exists() {
            return Err(Error::DirectoryDoesNotExist(
                self.path.as_os_str().to_owned(),
            ));
        }

        if Manifest::exists_at(&self.path) {
            return Err(Error::CircuitAlreadyInitialized(
                self.path.as_os_str().to_owned(),
            ));
        }
        Manifest::new(&project_name, project_type)
            .write_to(&self.path)
            .map_err(Error::ManifestFile)?;

        SourceDirectory::create(&self.path).map_err(Error::SourceDirectory)?;

        match project_type {
            ProjectType::Circuit => {
                if !CircuitFile::exists_at(&self.path) {
                    CircuitFile::new(&project_name)
                        .write_to(&self.path)
                        .map_err(Error::CircuitFile)?;
                }
            }
            ProjectType::Contract => {
                if !ContractFile::exists_at(&self.path, &project_name) {
                    ContractFile::new(&project_name)
                        .write_to(&self.path)
                        .map_err(Error::ContractFile)?;
                }
            }
        }

        log::info!(
            "     Created project `{}` at {}",
            project_name,
            self.path.to_string_lossy(),
        );
        Ok(())
    }
}
