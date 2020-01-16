//!
//! The Zargo `init` command.
//!

use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io;
use std::path::PathBuf;

use failure::Fail;
use std::io::Write;
use structopt::StructOpt;

use crate::templates;

#[derive(Debug, StructOpt)]
#[structopt(about = "Creates a new circuit")]
pub struct Command {
    #[structopt(short = "q", long = "quiet", help = "No output printed to stdout")]
    quiet: bool,
    #[structopt(short = "v", long = "verbose", help = "Use verbose output")]
    verbose: bool,
    #[structopt(
        long = "name",
        help = "Set the resulting circuit name, defaults to the directory name"
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
    #[fail(display = "manifest file {:?} creating: {}", _0, _1)]
    CreatingZargoManifestFile(OsString, io::Error),
    #[fail(display = "manifest file {:?} template writing: {}", _0, _1)]
    WritingZargoManifestFileTemplate(OsString, io::Error),
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

        if !self.path.exists() {
            return Err(Error::DirectoryDoesNotExist(
                self.path.as_os_str().to_owned(),
            ));
        }

        let mut zargo_manifest_file_path = self.path.clone();
        zargo_manifest_file_path.push(PathBuf::from(crate::constants::CIRCUIT_MANIFEST_FILE_NAME));
        if zargo_manifest_file_path.exists() {
            return Err(Error::CircuitAlreadyInitialized(
                zargo_manifest_file_path.as_os_str().to_owned(),
            ));
        }

        let mut zargo_file = File::create(&zargo_manifest_file_path).map_err(|error| {
            Error::CreatingZargoManifestFile(zargo_manifest_file_path.as_os_str().to_owned(), error)
        })?;
        zargo_file
            .write_all(templates::manifest_template(&circuit_name).as_bytes())
            .map_err(|error| {
                Error::WritingZargoManifestFileTemplate(
                    zargo_manifest_file_path.as_os_str().to_owned(),
                    error,
                )
            })?;

        let mut source_directory_path = self.path.clone();
        source_directory_path.push(PathBuf::from(crate::constants::CIRCUIT_SOURCE_DIRECTORY));
        fs::create_dir_all(&source_directory_path).map_err(|error| {
            Error::CreatingSourceDirectory(source_directory_path.as_os_str().to_owned(), error)
        })?;

        let mut source_main_file_path = source_directory_path;
        source_main_file_path.push(PathBuf::from(crate::constants::CIRCUIT_MAIN_FILE_NAME));
        if !source_main_file_path.exists() {
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
        }

        if !self.quiet {
            log::info!(
                "The directory '{}' has been initialized with circuit '{}'",
                self.path.to_string_lossy(),
                circuit_name,
            );
        }
        Ok(())
    }
}
