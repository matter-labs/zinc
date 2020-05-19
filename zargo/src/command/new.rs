//!
//! The `new` command.
//!

use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::str::FromStr;

use failure::Fail;
use structopt::StructOpt;

use crate::directory::source::circuit::Circuit as CircuitFile;
use crate::directory::source::circuit::Error as CircuitFileError;
use crate::directory::source::contract::Contract as ContractFile;
use crate::directory::source::contract::Error as ContractFileError;
use crate::directory::source::Directory as SourceDirectory;
use crate::directory::source::Error as SourceDirectoryError;
use crate::manifest::project_type::ProjectType;
use crate::manifest::Error as ManifestError;
use crate::manifest::Manifest;

#[derive(Debug, StructOpt)]
#[structopt(about = "Creates a new project directory")]
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

    #[structopt(parse(from_os_str))]
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
        display = "project type must be either `circuit` or `contract`, found `{}`",
        _0
    )]
    ProjectTypeInvalid(String),
    #[fail(display = "directory {:?} already exists", _0)]
    DirectoryAlreadyExists(OsString),
    #[fail(display = "root directory {:?} creating: {}", _0, _1)]
    CreatingRootDirectory(OsString, io::Error),
    #[fail(display = "manifest file {}", _0)]
    ManifestFile(ManifestError),
    #[fail(display = "source directory {}", _0)]
    SourceDirectory(SourceDirectoryError),
    #[fail(display = "main file {}", _0)]
    CircuitFile(CircuitFileError),
    #[fail(display = "contract file {}", _0)]
    ContractFile(ContractFileError),
}

impl Command {
    pub fn execute(mut self) -> Result<(), Error> {
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
            self.path.to_string_lossy()
        );
        Ok(())
    }
}
