//!
//! The Zargo package manager `init` subcommand.
//!

use std::ffi::OsString;

use failure::Fail;

use crate::error::directory::Error as DirectoryError;
use crate::error::file::Error as FileError;

///
/// The Zargo package manager `init` subcommand error.
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
    #[fail(display = "manifest {}", _0)]
    Manifest(zinc_manifest::Error),
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
