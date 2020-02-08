//!
//! The Zargo `new` command.
//!

use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io;
use std::path::PathBuf;

use failure::Fail;
use std::io::Write;
use structopt::StructOpt;

use crate::manifest::Error as ManifestError;
use crate::manifest::Manifest;
use crate::templates;

#[derive(Debug, StructOpt)]
#[structopt(about = "Creates a new circuit project directory")]
pub struct Command {
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
    verbose: usize,

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
    #[fail(display = "manifest file {:?}: {}", _0, _1)]
    ManifestFile(OsString, ManifestError),
    #[fail(display = "source directory {:?} creating: {}", _0, _1)]
    CreatingSourceDirectory(OsString, io::Error),
    #[fail(display = "source file {:?} creating: {}", _0, _1)]
    CreatingSourceMainFile(OsString, io::Error),
    #[fail(display = "source file {:?} template writing: {}", _0, _1)]
    WritingSourceMainFileTemplate(OsString, io::Error),
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
            .map_err(|error| Error::ManifestFile(self.path.as_os_str().to_owned(), error))?;

        let mut source_directory_path = self.path.clone();
        source_directory_path.push(PathBuf::from(crate::constants::CIRCUIT_DIRECTORY_SOURCE));
        fs::create_dir_all(&source_directory_path).map_err(|error| {
            Error::CreatingSourceDirectory(source_directory_path.as_os_str().to_owned(), error)
        })?;

        let mut source_main_file_path = source_directory_path;
        source_main_file_path.push(PathBuf::from(crate::constants::CIRCUIT_FILE_NAME_MAIN));
        let mut main_file = File::create(&source_main_file_path).map_err(|error| {
            Error::CreatingSourceMainFile(source_main_file_path.as_os_str().to_owned(), error)
        })?;
        main_file
            .write_all(templates::main_template(&circuit_name).as_bytes())
            .map_err(|error| {
                Error::WritingSourceMainFileTemplate(
                    source_main_file_path.as_os_str().to_owned(),
                    error,
                )
            })?;

        log::info!(
            "     Created circuit `{}` at {}",
            circuit_name,
            self.path.to_string_lossy()
        );
        Ok(())
    }
}
