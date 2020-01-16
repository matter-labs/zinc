//!
//! The Zargo `clean` command.
//!

use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

use crate::manifest::Error as ManifestError;
use crate::manifest::Manifest;

#[derive(Debug, StructOpt)]
#[structopt(about = "Removes the build directory")]
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
    #[fail(display = "build directory {:?} removing: {}", _0, _1)]
    BuildDirectoryRemoving(OsString, io::Error),
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

        let mut build_directory_path = project_path.clone();
        build_directory_path.push(crate::constants::CIRCUIT_BUILD_DIRECTORY);
        if build_directory_path.exists() {
            fs::remove_dir_all(&build_directory_path).map_err(|error| {
                Error::BuildDirectoryRemoving(build_directory_path.as_os_str().to_owned(), error)
            })?;
        }

        if !self.quiet {
            log::info!(
                "The '{}' circuit directory '{}' has been cleaned up",
                manifest.circuit.name,
                project_path.to_string_lossy()
            );
        }
        Ok(())
    }
}
