//!
//! The `new` command.
//!

use std::ffi::OsString;
use std::fs;
use std::io;
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
#[structopt(about = "Creates a new circuit project directory")]
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

    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(
        display = "circuit name is missing and cannot be inferred from path {:?}",
        _0
    )]
    ProjectNameInvalid(OsString),
    #[fail(display = "directory {:?} already exists", _0)]
    DirectoryAlreadyExists(OsString),
    #[fail(display = "root directory {:?} creating: {}", _0, _1)]
    CreatingRootDirectory(OsString, io::Error),
    #[fail(display = "manifest directory.file {}", _0)]
    ManifestFile(ManifestError),
    #[fail(display = "source directory {}", _0)]
    SourceDirectory(SourceDirectoryError),
    #[fail(display = "main directory.file {}", _0)]
    MainFile(MainFileError),
}

impl Command {
    pub fn execute(mut self) -> Result<(), Error> {
        let circuit_name = self.name.take().unwrap_or(
            self.path
                .file_stem()
                .ok_or_else(|| Error::ProjectNameInvalid(self.path.as_os_str().to_owned()))?
                .to_string_lossy()
                .to_string(),
        );

        if self.path.exists() {
            return Err(Error::DirectoryAlreadyExists(
                self.path.as_os_str().to_owned(),
            ));
        }
        fs::create_dir_all(&self.path).map_err(|error| {
            Error::CreatingRootDirectory(self.path.as_os_str().to_owned(), error)
        })?;

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
            self.path.to_string_lossy()
        );
        Ok(())
    }
}
