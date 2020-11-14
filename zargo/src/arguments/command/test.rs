//!
//! The Zargo package manager `test` subcommand.
//!

use std::convert::TryFrom;
use std::path::PathBuf;

use structopt::StructOpt;

use zinc_manifest::Manifest;

use crate::executable::compiler::Compiler;
use crate::executable::virtual_machine::VirtualMachine;
use crate::project::target::deps::Directory as TargetDependenciesDirectory;
use crate::project::target::Directory as TargetDirectory;

///
/// The Zargo package manager `test` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Runs the project unit tests")]
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

///
/// The unit test summary.
///
#[derive(Default)]
pub struct Summary {
    pub passed: u8,
    pub failed: u8,
    pub invalid: u8,
    pub ignored: u8,
}

impl Command {
    ///
    /// Executes the command.
    ///
    pub fn execute(self) -> anyhow::Result<()> {
        let manifest = Manifest::try_from(&self.manifest_path)?;

        let mut manifest_path = self.manifest_path.clone();
        if manifest_path.is_file() {
            manifest_path.pop();
        }

        TargetDirectory::create(&manifest_path, true)?;
        let target_directory_path = TargetDirectory::path(&manifest_path, true);
        let mut binary_path = target_directory_path;
        binary_path.push(format!(
            "{}.{}",
            zinc_const::file_name::BINARY,
            zinc_const::extension::BINARY
        ));
        TargetDependenciesDirectory::create(&manifest_path)?;

        Compiler::build_release(
            self.verbosity,
            manifest.project.name.as_str(),
            &manifest.project.version,
            &manifest_path,
            true,
        )?;

        VirtualMachine::test(self.verbosity, &binary_path)?;

        Ok(())
    }
}
