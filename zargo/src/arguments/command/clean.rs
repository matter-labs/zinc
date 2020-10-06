//!
//! The Zargo package manager `clean` subcommand.
//!

use std::convert::TryFrom;
use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

use crate::arguments::command::IExecutable;
use crate::error::directory::Error as DirectoryError;
use crate::error::file::Error as FileError;
use crate::project::build::Directory as BuildDirectory;
use crate::project::data::Directory as DataDirectory;
use crate::project::manifest::Manifest as ManifestFile;

///
/// The Zargo package manager `clean` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Removes the project build artifacts")]
pub struct Command {
    /// Prints more logs, if passed several times.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbosity: usize,

    /// The path to the Zargo project manifest file.
    #[structopt(
        long = "manifest-path",
        parse(from_os_str),
        default_value = zinc_const::path::MANIFEST,
    )]
    pub manifest_path: PathBuf,
}

///
/// The Zargo package manager `clean` subcommand error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The manifest file error.
    #[fail(display = "manifest file {}", _0)]
    ManifestFile(FileError<toml::de::Error>),
    /// The project binary build directory error.
    #[fail(display = "build directory {}", _0)]
    BuildDirectory(DirectoryError),
    /// The project template, keys, and other auxiliary data directory error.
    #[fail(display = "data directory {}", _0)]
    DataDirectory(DirectoryError),
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(self) -> Result<(), Self::Error> {
        let _manifest = ManifestFile::try_from(&self.manifest_path).map_err(Error::ManifestFile)?;

        let mut manifest_path = self.manifest_path;
        if manifest_path.is_file() {
            manifest_path.pop();
        }

        DataDirectory::remove(&manifest_path).map_err(Error::DataDirectory)?;
        BuildDirectory::remove(&manifest_path).map_err(Error::BuildDirectory)?;

        Ok(())
    }
}
