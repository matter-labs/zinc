//!
//! The Zargo package manager `clean` subcommand.
//!

pub mod error;

use std::convert::TryFrom;
use std::path::PathBuf;

use structopt::StructOpt;

use zinc_manifest::Manifest;

use crate::project::build::Directory as BuildDirectory;
use crate::project::data::Directory as DataDirectory;

use self::error::Error;

///
/// The Zargo package manager `clean` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Removes the project build artifacts")]
pub struct Command {
    /// Prints more logs, if passed several times.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbosity: usize,

    /// The path to the Zinc project manifest file.
    #[structopt(
        long = "manifest-path",
        parse(from_os_str),
        default_value = "./Zargo.toml"
    )]
    pub manifest_path: PathBuf,
}

impl Command {
    ///
    /// Executes the command.
    ///
    pub fn execute(self) -> Result<(), Error> {
        let _manifest = Manifest::try_from(&self.manifest_path).map_err(Error::Manifest)?;

        let mut manifest_path = self.manifest_path;
        if manifest_path.is_file() {
            manifest_path.pop();
        }

        DataDirectory::remove(&manifest_path).map_err(Error::DataDirectory)?;
        BuildDirectory::remove(&manifest_path).map_err(Error::BuildDirectory)?;

        Ok(())
    }
}
