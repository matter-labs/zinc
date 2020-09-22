//!
//! The Zargo project manager `build` subcommand.
//!

use std::convert::TryFrom;
use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

use crate::arguments::command::IExecutable;
use crate::error::directory::Error as DirectoryError;
use crate::error::file::Error as FileError;
use crate::executable::compiler::Compiler;
use crate::executable::compiler::Error as CompilerError;
use crate::project::build::Directory as BuildDirectory;
use crate::project::data::private_key::PrivateKey as PrivateKeyFile;
use crate::project::data::Directory as DataDirectory;
use crate::project::manifest::project_type::ProjectType;
use crate::project::manifest::Manifest as ManifestFile;
use crate::project::source::Directory as SourceDirectory;

///
/// The Zargo project manager `build` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Builds the project at the given path")]
pub struct Command {
    /// The logging level value, which helps the logger to set the logging level.
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
    pub verbosity: usize,

    /// The path to the Zargo project manifest file.
    #[structopt(
        long = "manifest-path",
        help = "Path to Zargo.toml",
        default_value = zinc_const::path::MANIFEST,
    )]
    pub manifest_path: PathBuf,

    /// Whether to build the release version.
    #[structopt(long = "release", help = "Build the release version")]
    pub is_release: bool,
}

///
/// The Zargo project manager `build` subcommand error.
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
    /// The private key file generation error.
    #[fail(display = "private key file {}", _0)]
    PrivateKeyFile(FileError),
    /// The compiler process error.
    #[fail(display = "compiler {}", _0)]
    Compiler(CompilerError),
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(self) -> Result<(), Self::Error> {
        let manifest = ManifestFile::try_from(&self.manifest_path).map_err(Error::ManifestFile)?;

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
