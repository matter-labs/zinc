//!
//! The Zargo package manager `test` subcommand.
//!

use std::convert::TryFrom;
use std::path::PathBuf;

use structopt::StructOpt;

use zinc_manifest::Manifest;

use crate::executable::compiler::Compiler;
use crate::executable::virtual_machine::VirtualMachine;
use crate::project::build::Directory as BuildDirectory;
use crate::project::data::Directory as DataDirectory;
use crate::project::source::Directory as SourceDirectory;

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

        let source_directory_path = SourceDirectory::path(&manifest_path);

        let data_directory_path = DataDirectory::path(&manifest_path);

        BuildDirectory::create(&manifest_path)?;
        let build_directory_path = BuildDirectory::path(&manifest_path);
        let mut binary_path = build_directory_path;
        binary_path.push(format!(
            "{}.{}",
            zinc_const::file_name::BINARY,
            zinc_const::extension::BINARY
        ));

        Compiler::build_release(
            self.verbosity,
            manifest.project.name.as_str(),
            manifest.project.version.as_str(),
            &manifest_path,
            &data_directory_path,
            &source_directory_path,
            &binary_path,
            true,
        )?;

        VirtualMachine::test(self.verbosity, &binary_path)?;

        Ok(())
    }
}
