//!
//! The `init` command.
//!

use std::ffi::OsString;
use std::path::PathBuf;
use std::str::FromStr;

use failure::Fail;
use structopt::StructOpt;

use crate::directory::source::contract::Contract as ContractFile;
use crate::directory::source::contract::Error as ContractFileError;
use crate::directory::source::main::Error as MainFileError;
use crate::directory::source::main::Main as MainFile;
use crate::directory::source::Directory as SourceDirectory;
use crate::directory::source::Error as SourceDirectoryError;
use crate::manifest::project_type::ProjectType;
use crate::manifest::Error as ManifestError;
use crate::manifest::Manifest;

#[derive(Debug, StructOpt)]
#[structopt(about = "Initializes a new project in the specified directory")]
pub struct Command {
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
    verbosity: usize,

    #[structopt(
        long = "name",
        help = "Set the project name, defaults to the directory name"
    )]
    name: Option<String>,

    #[structopt(
        long = "type",
        help = "Set the project type, either 'circuit' or 'contract'"
    )]
    r#type: String,

    #[structopt(parse(from_os_str), default_value = "./")]
    path: PathBuf,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(
        display = "project name is missing and cannot be inferred from path {:?}",
        _0
    )]
    ProjectNameInvalid(OsString),
    #[fail(
        display = "project type must be either 'circuit' or 'contract', found {}",
        _0
    )]
    ProjectTypeInvalid(String),
    #[fail(display = "directory {:?} does not exist", _0)]
    DirectoryDoesNotExist(OsString),
    #[fail(display = "project at path {:?} is already initialized", _0)]
    CircuitAlreadyInitialized(OsString),
    #[fail(display = "manifest file {}", _0)]
    ManifestFile(ManifestError),
    #[fail(display = "source directory {}", _0)]
    SourceDirectory(SourceDirectoryError),
    #[fail(display = "main file {}", _0)]
    MainFile(MainFileError),
    #[fail(display = "contract file {}", _0)]
    ContractFile(ContractFileError),
}

impl Command {
    pub fn execute(mut self) -> Result<(), Error> {
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
                if !MainFile::exists_at(&self.path) {
                    MainFile::new(&project_name)
                        .write_to(&self.path)
                        .map_err(Error::MainFile)?;
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
