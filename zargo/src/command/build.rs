//!
//! The Zargo `build` command.
//!

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
#[structopt(about = "Builds the current circuit")]
pub struct Command {
    #[structopt(short = "q", long = "quiet", help = "No output printed to stdout")]
    quiet: bool,
    #[structopt(short = "v", long = "verbose", help = "Use verbose output")]
    verbose: bool,
    #[structopt(
        long = "manifest-path",
        help = "Path to Zargo.toml",
        default_value = "./Zargo.toml"
    )]
    manifest_path: PathBuf,
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
    pub fn execute(mut self) -> Result<(), Error> {
        let mut project_path = self.manifest_path.clone();
        if !self
            .manifest_path
            .ends_with(crate::constants::CIRCUIT_MANIFEST_FILE_NAME)
        {
            self.manifest_path
                .push(crate::constants::CIRCUIT_MANIFEST_FILE_NAME);
        } else {
            project_path.pop();
        }

        let manifest = Manifest::new(&self.manifest_path).map_err(|error| {
            Error::ManifestFile(self.manifest_path.as_os_str().to_owned(), error)
        })?;

        let mut source_directory_path = project_path.clone();
        source_directory_path.push(PathBuf::from(crate::constants::CIRCUIT_SOURCE_DIRECTORY));
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
            if source_file_extension != crate::constants::ZINC_SOURCE_FILE_EXTENSION {
                return Err(Error::InvalidSourceFileExtension(
                    source_file_path.as_os_str().to_owned(),
                    source_file_extension.to_owned(),
                ));
            }

            source_file_paths.push(source_file_path);
        }

        let mut build_directory_path = project_path;
        build_directory_path.push(PathBuf::from(crate::constants::CIRCUIT_BUILD_DIRECTORY));
        fs::create_dir_all(&build_directory_path).map_err(|error| {
            Error::CreatingBuildDirectory(build_directory_path.as_os_str().to_owned(), error)
        })?;

        let mut build_input_template_path = build_directory_path.clone();
        build_input_template_path.push(crate::constants::CIRCUIT_INPUT_TEMPLATE_DEFAULT_FILE_NAME);

        let mut build_output_template_path = build_directory_path.clone();
        build_output_template_path
            .push(crate::constants::CIRCUIT_RESULT_TEMPLATE_DEFAULT_FILE_NAME);

        let mut build_binary_path = build_directory_path.clone();
        build_binary_path.push(crate::constants::CIRCUIT_BINARY_DEFAULT_FILE_NAME);

        let mut compiler_process =
            process::Command::new(crate::constants::ZINC_COMPILER_BINARY_NAME)
                .arg("--input-json")
                .arg(&build_input_template_path)
                .arg("--output-json")
                .arg(&build_output_template_path)
                .arg("--output")
                .arg(&build_binary_path)
                .args(&source_file_paths)
                .spawn()
                .map_err(Error::CompilerProcessSpawning)?;
        let compiler_process_status = compiler_process
            .wait()
            .map_err(Error::CompilerProcessWaiting)?;
        if !compiler_process_status.success() {
            return Err(Error::CompilerProcessFailure(compiler_process_status));
        }

        if !self.quiet {
            log::info!(
                "The '{}' circuit has been built to '{}'",
                manifest.circuit.name,
                build_directory_path.to_string_lossy()
            );
        }
        Ok(())
    }
}
