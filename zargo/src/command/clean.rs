//!
//! The Zargo `clean` command.
//!

use std::convert::TryFrom;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

use crate::manifest::Error as ManifestError;
use crate::manifest::Manifest;

#[derive(Debug, StructOpt)]
#[structopt(about = "Cleans up the circuit project")]
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
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "manifest file {:?} error: {}", _0, _1)]
    ManifestFile(OsString, ManifestError),
    #[fail(display = "build directory {:?} removing: {}", _0, _1)]
    BuildDirectoryRemoving(OsString, io::Error),
}

impl Command {
    pub fn execute(self) -> Result<(), Error> {
        let _manifest = Manifest::try_from(&self.manifest_path).map_err(|error| {
            Error::ManifestFile(self.manifest_path.as_os_str().to_owned(), error)
        })?;

        let mut project_path = self.manifest_path;
        if project_path.is_file() {
            project_path.pop();
        }

        let mut build_directory_path = project_path;
        build_directory_path.push(crate::constants::CIRCUIT_DIRECTORY_BUILD);
        if build_directory_path.exists() {
            fs::remove_dir_all(&build_directory_path).map_err(|error| {
                Error::BuildDirectoryRemoving(build_directory_path.as_os_str().to_owned(), error)
            })?;
        }

        Ok(())
    }
}
