//!
//! The `init` command.
//!

use std::ffi::OsString;
use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

use crate::directory::source::main::Error as MainFileError;
use crate::directory::source::main::Main as MainFile;
use crate::directory::source::Directory as SourceDirectory;
use crate::directory::source::Error as SourceDirectoryError;
use crate::manifest::Error as ManifestError;
use crate::manifest::Manifest;

#[derive(Debug, StructOpt)]
#[structopt(about = "Initializes a new circuit in the specified directory")]
pub struct Command {
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
    verbosity: usize,

    #[structopt(
        long = "name",
        help = "Set the outputing circuit name, defaults to the directory name"
    )]
    name: Option<String>,

    #[structopt(parse(from_os_str), default_value = "./")]
    path: PathBuf,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(
        display = "circuit name is missing and cannot be inferred from path {:?}",
        _0
    )]
    ProjectNameInvalid(OsString),
    #[fail(display = "directory {:?} does not exist", _0)]
    DirectoryDoesNotExist(OsString),
    #[fail(display = "circuit at path {:?} is already initialized", _0)]
    CircuitAlreadyInitialized(OsString),
    #[fail(display = "manifest directory.file {}", _0)]
    ManifestFile(ManifestError),
    #[fail(display = "source directory {}", _0)]
    SourceDirectory(SourceDirectoryError),
    #[fail(display = "main directory.file {}", _0)]
    MainFile(MainFileError),
}

impl Command {
    pub fn execute(mut self) -> Result<(), Error> {
        let circuit_name = match self.name.take() {
            Some(name) => name,
            None => self
                .path
                .file_stem()
                .ok_or_else(|| Error::ProjectNameInvalid(self.path.as_os_str().to_owned()))?
                .to_string_lossy()
                .to_string(),
        };

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
        Manifest::new(&circuit_name)
            .write_to(&self.path)
            .map_err(Error::ManifestFile)?;

        SourceDirectory::create(&self.path).map_err(Error::SourceDirectory)?;

        if !MainFile::exists_at(&self.path) {
            MainFile::new(&circuit_name)
                .write_to(&self.path)
                .map_err(Error::MainFile)?;
        }

        log::info!(
            "     Created circuit `{}` at {}",
            circuit_name,
            self.path.to_string_lossy(),
        );
        Ok(())
    }
}
