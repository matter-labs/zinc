//!
//! The Zargo package manager `clean` subcommand.
//!

use std::convert::TryFrom;
use std::path::PathBuf;

use structopt::StructOpt;

use crate::project::data::Directory as DataDirectory;
use crate::project::target::Directory as TargetDirectory;

///
/// The Zargo package manager `clean` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Removes the project build artifacts")]
pub struct Command {
    /// Prints more logs, if passed several times.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbosity: usize,

    /// Suppresses output, if set.
    #[structopt(short = "q", long = "quiet")]
    pub quiet: bool,

    /// The path to the Zinc project manifest file.
    #[structopt(
        long = "manifest-path",
        parse(from_os_str),
        default_value = "./Zargo.toml"
    )]
    pub manifest_path: PathBuf,

    /// Removes the dependencies directory, if set.
    #[structopt(long = "dependencies")]
    pub remove_dependencies: bool,
}

impl Command {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        verbosity: usize,
        quiet: bool,
        manifest_path: PathBuf,
        remove_dependencies: bool,
    ) -> Self {
        Self {
            verbosity,
            quiet,
            manifest_path,
            remove_dependencies,
        }
    }

    ///
    /// Executes the command.
    ///
    pub fn execute(self) -> anyhow::Result<()> {
        let _manifest = zinc_project::Manifest::try_from(&self.manifest_path)?;

        let mut manifest_path = self.manifest_path;
        if manifest_path.is_file() {
            manifest_path.pop();
        }

        DataDirectory::remove(&manifest_path)?;
        TargetDirectory::remove(&manifest_path, self.remove_dependencies)?;

        Ok(())
    }
}
