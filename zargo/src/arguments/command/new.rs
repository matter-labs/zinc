//!
//! The Zargo package manager `new` subcommand.
//!

use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::str::FromStr;

use colored::Colorize;
use failure::Fail;
use structopt::StructOpt;

use crate::arguments::command::IExecutable;
use crate::error::directory::Error as DirectoryError;
use crate::error::file::Error as FileError;
use crate::project::manifest::project_type::ProjectType;
use crate::project::manifest::Manifest as ManifestFile;
use crate::project::source::circuit::Circuit as CircuitFile;
use crate::project::source::contract::Contract as ContractFile;
use crate::project::source::Directory as SourceDirectory;

///
/// The Zargo package manager `new` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Creates a new project in the specified directory")]
pub struct Command {
    /// Prints more logs, if passed several times.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbosity: usize,

    /// Sets the project name, defaults to the directory name.
    #[structopt(long = "name")]
    pub name: Option<String>,

    /// Sets the project type, either 'circuit' or 'contract'.
    #[structopt(long = "type")]
    pub r#type: String,

    /// The path to the project directory to initialize.
    #[structopt(parse(from_os_str))]
    pub path: PathBuf,
}

///
/// The Zargo package manager `new` subcommand error.
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
        display = "project type must be either `circuit` or `contract`, but found `{}`",
        _0
    )]
    ProjectTypeInvalid(String),
    /// The project directory already exists. Use `init` instead.
    #[fail(
        display = "directory {:?} already exists. To initialize it with a project, use `zargo init`",
        _0
    )]
    DirectoryAlreadyExists(OsString),
    /// The project directory creating error.
    #[fail(display = "root directory {:?} creating: {}", _0, _1)]
    CreatingRootDirectory(OsString, io::Error),
    /// The manifest file error.
    #[fail(display = "manifest file {}", _0)]
    ManifestFile(FileError<toml::de::Error>),
    /// The project source code directory error.
    #[fail(display = "source directory {}", _0)]
    SourceDirectory(DirectoryError),
    /// The circuit source code entry point file generation error.
    #[fail(display = "main file {}", _0)]
    CircuitFile(FileError),
    /// The contract source code entry point file generation error.
    #[fail(display = "contract file {}", _0)]
    ContractFile(FileError),
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(mut self) -> Result<(), Error> {
        let project_name = self.name.take().unwrap_or(
            self.path
                .file_stem()
                .ok_or_else(|| Error::ProjectNameInvalid(self.path.as_os_str().to_owned()))?
                .to_string_lossy()
                .to_string(),
        );

        let project_type =
            ProjectType::from_str(self.r#type.as_str()).map_err(Error::ProjectTypeInvalid)?;

        if self.path.exists() {
            return Err(Error::DirectoryAlreadyExists(
                self.path.as_os_str().to_owned(),
            ));
        }
        fs::create_dir_all(&self.path).map_err(|error| {
            Error::CreatingRootDirectory(self.path.as_os_str().to_owned(), error)
        })?;

        ManifestFile::new(&project_name, project_type)
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
                if !ContractFile::exists_at(&self.path) {
                    ContractFile::new(&project_name)
                        .write_to(&self.path)
                        .map_err(Error::ContractFile)?;
                }
            }
        }

        eprintln!(
            "     {} {} `{}`",
            "Created".bright_green(),
            project_type,
            project_name,
        );

        Ok(())
    }
}
