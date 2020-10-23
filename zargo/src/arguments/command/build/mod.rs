//!
//! The Zargo package manager `build` subcommand.
//!

pub mod error;

use std::convert::TryFrom;
use std::path::PathBuf;

use structopt::StructOpt;

use zinc_manifest::Manifest;
use zinc_manifest::ProjectType;

use crate::executable::compiler::Compiler;
use crate::project::build::Directory as BuildDirectory;
use crate::project::data::private_key::PrivateKey as PrivateKeyFile;
use crate::project::data::Directory as DataDirectory;
use crate::project::source::Directory as SourceDirectory;

use self::error::Error;

///
/// The Zargo package manager `build` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Builds the project at the given path")]
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

    /// Builds the release version.
    #[structopt(long = "release")]
    pub is_release: bool,
}

impl Command {
    ///
    /// Executes the command.
    ///
    pub fn execute(self) -> Result<(), Error> {
        let manifest = Manifest::try_from(&self.manifest_path).map_err(Error::Manifest)?;

        let mut manifest_path = self.manifest_path.clone();
        if manifest_path.is_file() {
            manifest_path.pop();
        }

        let source_directory_path = SourceDirectory::path(&manifest_path);

        DataDirectory::create(&manifest_path).map_err(Error::DataDirectory)?;
        let data_directory_path = DataDirectory::path(&manifest_path);

        BuildDirectory::create(&manifest_path).map_err(Error::BuildDirectory)?;
        let build_directory_path = BuildDirectory::path(&manifest_path);
        let mut binary_path = build_directory_path;
        binary_path.push(format!(
            "{}.{}",
            zinc_const::file_name::BINARY,
            zinc_const::extension::BINARY
        ));

        if let ProjectType::Contract = manifest.project.r#type {
            if !PrivateKeyFile::exists_at(&data_directory_path) {
                PrivateKeyFile::default()
                    .write_to(&data_directory_path)
                    .map_err(Error::PrivateKeyFile)?;
            }
        }

        if self.is_release {
            Compiler::build_release(
                self.verbosity,
                manifest.project.name.as_str(),
                manifest.project.version.as_str(),
                &manifest_path,
                &data_directory_path,
                &source_directory_path,
                &binary_path,
                false,
            )
            .map_err(Error::Compiler)?;
        } else {
            Compiler::build_debug(
                self.verbosity,
                manifest.project.name.as_str(),
                manifest.project.version.as_str(),
                &manifest_path,
                &data_directory_path,
                &source_directory_path,
                &binary_path,
                false,
            )
            .map_err(Error::Compiler)?;
        }

        Ok(())
    }
}
