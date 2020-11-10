//!
//! The Zargo package manager `new` subcommand.
//!

use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use colored::Colorize;
use structopt::StructOpt;

use zinc_manifest::Manifest;
use zinc_manifest::ProjectType;

use crate::error::Error;
use crate::project::source::circuit::Circuit as CircuitFile;
use crate::project::source::contract::Contract as ContractFile;
use crate::project::source::Directory as SourceDirectory;

///
/// The Zargo package manager `new` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Creates a new project in the specified directory")]
pub struct Command {
    /// Prints more logs, if passed several times.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbosity: usize,

    /// Sets the project name, defaults to the directory name.
    #[structopt(long = "name")]
    pub name: Option<String>,

    /// Sets the project type, either 'circuit' or 'contract'.
    #[structopt(long = "type")]
    pub r#type: String,

    /// The path to the project directory to initialize.
    #[structopt(parse(from_os_str))]
    pub path: PathBuf,
}

impl Command {
    ///
    /// Executes the command.
    ///
    pub fn execute(mut self) -> anyhow::Result<()> {
        let project_name = self.name.take().unwrap_or(
            self.path
                .file_stem()
                .ok_or_else(|| Error::ProjectNameInvalid(self.path.as_os_str().to_owned()))?
                .to_string_lossy()
                .to_string(),
        );

        let project_type =
            ProjectType::from_str(self.r#type.as_str()).map_err(Error::ProjectTypeInvalid)?;

        if self.path.exists() {
            anyhow::bail!(Error::DirectoryAlreadyExists(
                self.path.as_os_str().to_owned(),
            ));
        }
        fs::create_dir_all(&self.path)?;

        Manifest::new(&project_name, project_type).write_to(&self.path)?;

        SourceDirectory::create(&self.path)?;

        match project_type {
            ProjectType::Circuit => {
                if !CircuitFile::exists_at(&self.path) {
                    CircuitFile::new(&project_name).write_to(&self.path)?;
                }
            }
            ProjectType::Contract => {
                if !ContractFile::exists_at(&self.path) {
                    ContractFile::new(&project_name).write_to(&self.path)?;
                }
            }
        }

        eprintln!(
            "     {} {} `{}`",
            "Created".bright_green(),
            project_type,
            project_name,
        );

        Ok(())
    }
}
