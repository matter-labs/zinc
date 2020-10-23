//!
//! The Zargo package manager `new` subcommand.
//!

use std::ffi::OsString;
use std::io;

use failure::Fail;

use crate::error::directory::Error as DirectoryError;
use crate::error::file::Error as FileError;

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
