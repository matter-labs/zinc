//!
//! The Zargo `build` command.
//!

use std::convert::TryFrom;
use std::ffi::OsString;
use std::fs;
use std::fs::FileType;
use std::io;
use std::path::PathBuf;
use std::process;
use std::process::ExitStatus;

use failure::Fail;
use structopt::StructOpt;

use crate::manifest::Error as ManifestError;
use crate::manifest::Manifest;

#[derive(Debug, StructOpt)]
#[structopt(about = "Builds the circuit at the given path")]
pub struct Command {
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
    verbose: usize,

    #[structopt(
        long = "manifest-path",
        help = "Path to Zargo.toml",
        default_value = "./Zargo.toml"
    )]
    manifest_path: PathBuf,

    #[structopt(
        long = "circuit",
        help = "Path to the circuit binary file",
        default_value = "./build/default.znb"
    )]
    circuit: PathBuf,

    #[structopt(
        long = "witness",
        help = "Path to the witness JSON file",
        default_value = "./build/witness.json"
    )]
    witness: PathBuf,

    #[structopt(
        long = "public-data",
        help = "Path to the public data JSON file to write",
        default_value = "./build/public-data.json"
    )]
    public_data: PathBuf,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "manifest file {:?} error: {}", _0, _1)]
    ManifestFile(OsString, ManifestError),
    #[fail(display = "build directory {:?} creating: {}", _0, _1)]
    CreatingBuildDirectory(OsString, io::Error),
    #[fail(display = "source directory {:?} reading: {}", _0, _1)]
    ReadingSourceDirectory(OsString, io::Error),
    #[fail(display = "source file entry getting: {}", _0)]
    GettingSourceFileEntry(io::Error),
    #[fail(display = "source file {:?} type getting: {}", _0, _1)]
    GettingSourceFileType(OsString, io::Error),
    #[fail(display = "invalid source file {:?} type: {:?}", _0, _1)]
    InvalidSourceFileType(OsString, FileType),
    #[fail(display = "source file {:?} extension getting", _0)]
    GettingSourceFileExtension(OsString),
    #[fail(display = "invalid source file {:?} extension: {:?}", _0, _1)]
    InvalidSourceFileExtension(OsString, OsString),
    #[fail(display = "compiler process spawning: {}", _0)]
    CompilerProcessSpawning(io::Error),
    #[fail(display = "compiler process waiting: {}", _0)]
    CompilerProcessWaiting(io::Error),
    #[fail(display = "compiler process failure: {}", _0)]
    CompilerProcessFailure(ExitStatus),
}

impl Command {
    pub fn execute(self) -> Result<(), Error> {
        let _manifest = Manifest::try_from(&self.manifest_path).map_err(|error| {
            Error::ManifestFile(self.manifest_path.as_os_str().to_owned(), error)
        })?;

        let mut project_path = self.manifest_path.clone();
        if project_path.is_file() {
            project_path.pop();
        }

        let mut source_directory_path = project_path.clone();
        source_directory_path.push(PathBuf::from(crate::constants::CIRCUIT_DIRECTORY_SOURCE));
        let source_directory = fs::read_dir(&source_directory_path).map_err(|error| {
            Error::ReadingSourceDirectory(source_directory_path.as_os_str().to_owned(), error)
        })?;
        let mut source_file_paths = Vec::new();
        for source_file_entry in source_directory.into_iter() {
            let source_file_entry = source_file_entry.map_err(Error::GettingSourceFileEntry)?;
            let source_file_path = source_file_entry.path();

            let source_file_type = source_file_entry.file_type().map_err(|error| {
                Error::GettingSourceFileType(source_file_path.as_os_str().to_owned(), error)
            })?;
            if !source_file_type.is_file() {
                return Err(Error::InvalidSourceFileType(
                    source_file_path.as_os_str().to_owned(),
                    source_file_type,
                ));
            }

            let source_file_extension = source_file_path.extension().ok_or_else(|| {
                Error::GettingSourceFileExtension(source_file_path.as_os_str().to_owned())
            })?;
            if source_file_extension != crate::constants::ZINC_EXTENSION_SOURCE_FILE {
                return Err(Error::InvalidSourceFileExtension(
                    source_file_path.as_os_str().to_owned(),
                    source_file_extension.to_owned(),
                ));
            }

            source_file_paths.push(source_file_path);
        }

        let mut build_directory_path = project_path;
        build_directory_path.push(PathBuf::from(crate::constants::CIRCUIT_DIRECTORY_BUILD));
        fs::create_dir_all(&build_directory_path).map_err(|error| {
            Error::CreatingBuildDirectory(build_directory_path.as_os_str().to_owned(), error)
        })?;

        let mut compiler_process =
            process::Command::new(crate::constants::ZINC_BINARY_NAME_COMPILER)
                .args(vec!["-v"; self.verbose])
                .arg("--witness")
                .arg(&self.witness)
                .arg("--public-data")
                .arg(&self.public_data)
                .arg("--output")
                .arg(&self.circuit)
                .args(&source_file_paths)
                .spawn()
                .map_err(Error::CompilerProcessSpawning)?;
        let compiler_process_status = compiler_process
            .wait()
            .map_err(Error::CompilerProcessWaiting)?;
        if !compiler_process_status.success() {
            return Err(Error::CompilerProcessFailure(compiler_process_status));
        }

        Ok(())
    }
}
