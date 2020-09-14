//!
//! The Zargo project manager `publish` subcommand.
//!

use std::convert::TryFrom;
use std::path::PathBuf;

use colored::Colorize;
use failure::Fail;
use reqwest::Client as HttpClient;
use reqwest::Method;
use reqwest::Url;
use structopt::StructOpt;

use zinc_source::PublishRequestBody;
use zinc_source::PublishRequestQuery;
use zinc_source::Source;
use zinc_source::SourceError;

use crate::arguments::command::IExecutable;
use crate::directory::build::Directory as BuildDirectory;
use crate::directory::build::Error as BuildDirectoryError;
use crate::directory::data::Directory as DataDirectory;
use crate::directory::data::Error as DataDirectoryError;
use crate::directory::source::Directory as SourceDirectory;
use crate::executable::compiler::Compiler;
use crate::executable::compiler::Error as CompilerError;
use crate::executable::virtual_machine::Error as VirtualMachineError;
use crate::executable::virtual_machine::VirtualMachine;
use crate::file::arguments::Arguments as ArgumentsFile;
use crate::file::error::Error as FileError;
use crate::file::manifest::project_type::ProjectType;
use crate::file::manifest::Manifest as ManifestFile;
use crate::file::verifying_key::VerifyingKey as VerifyingKeyFile;

///
/// The Zargo project manager `publish` subcommand.
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

    /// The ID of the published contract.
    #[structopt(long = "id", help = "The ID of the published contract")]
    pub contract_id: i64,
}

///
/// The Zargo project manager `publish` subcommand error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The manifest file error.
    #[fail(display = "manifest file {}", _0)]
    ManifestFile(FileError<toml::de::Error>),
    /// The project is not a contract.
    #[fail(display = "not a contract")]
    NotAContract,
    /// The source code error.
    #[fail(display = "source code {}", _0)]
    Source(SourceError),
    /// The project binary build directory error.
    #[fail(display = "build directory {}", _0)]
    BuildDirectory(BuildDirectoryError),
    /// The project template, keys, and other auxiliary data directory error.
    #[fail(display = "data directory {}", _0)]
    DataDirectory(DataDirectoryError),
    /// The compiler process error.
    #[fail(display = "compiler {}", _0)]
    Compiler(CompilerError),
    /// The virtual machine process error.
    #[fail(display = "virtual machine {}", _0)]
    VirtualMachine(VirtualMachineError),
    /// The contract constructor input file error.
    #[fail(display = "constructor input file {}", _0)]
    ArgumentsFile(FileError<serde_json::Error>),
    /// The verifying key file error.
    #[fail(display = "verifying key file {}", _0)]
    VerifyingKeyFile(FileError),
    /// The publish HTTP request error.
    #[fail(display = "HTTP request: {}", _0)]
    HttpRequest(reqwest::Error),
    /// The smart contract server failure.
    #[fail(display = "action failed: {}", _0)]
    ActionFailed(String),
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(self) -> Result<(), Self::Error> {
        let manifest = ManifestFile::try_from(&self.manifest_path).map_err(Error::ManifestFile)?;

        match manifest.project.r#type {
            ProjectType::Contract => {}
            _ => return Err(Error::NotAContract),
        }

        let mut manifest_path = self.manifest_path;
        if manifest_path.is_file() {
            manifest_path.pop();
        }

        let source_directory_path = SourceDirectory::path(&manifest_path);
        let source = Source::try_from_path(&source_directory_path, true).map_err(Error::Source)?;

        DataDirectory::create(&manifest_path).map_err(Error::DataDirectory)?;
        let data_directory_path = DataDirectory::path(&manifest_path);
        let mut arguments_path = data_directory_path.clone();
        arguments_path.push(format!(
            "{}_{}.{}",
            zinc_const::file_name::WITNESS,
            zinc_const::zandbox::CONTRACT_CONSTRUCTOR_NAME,
            zinc_const::extension::JSON,
        ));
        let mut proving_key_path = data_directory_path.clone();
        proving_key_path.push(zinc_const::file_name::PROVING_KEY);
        let mut verifying_key_path = data_directory_path.clone();
        verifying_key_path.push(zinc_const::file_name::VERIFYING_KEY.to_owned());

        BuildDirectory::create(&manifest_path).map_err(Error::BuildDirectory)?;
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
            &data_directory_path,
            &source_directory_path,
            &binary_path,
            false,
        )
        .map_err(Error::Compiler)?;

        let arguments = ArgumentsFile::try_from_path(
            &arguments_path,
            zinc_const::zandbox::CONTRACT_CONSTRUCTOR_NAME,
        )
        .map_err(Error::ArgumentsFile)?;

        if !verifying_key_path.exists() {
            VirtualMachine::setup_contract(
                self.verbosity,
                &binary_path,
                zinc_const::zandbox::CONTRACT_CONSTRUCTOR_NAME,
                &proving_key_path,
                &verifying_key_path,
            )
            .map_err(Error::VirtualMachine)?;
        }

        let verifying_key =
            VerifyingKeyFile::try_from(&verifying_key_path).map_err(Error::VerifyingKeyFile)?;

        eprintln!(
            "   {} {} v{} with ID {}",
            "Uploading".bright_green(),
            manifest.project.name,
            manifest.project.version,
            self.contract_id,
        );
        let http_client = HttpClient::new();
        let mut http_response = http_client
            .execute(
                http_client
                    .request(
                        Method::POST,
                        Url::parse_with_params(
                            zinc_const::zandbox::CONTRACT_PUBLISH_URL,
                            PublishRequestQuery::new(
                                self.contract_id,
                                manifest.project.name,
                                manifest.project.version,
                            )
                            .into_vec(),
                        )
                        .expect(zinc_const::panic::DATA_SERIALIZATION),
                    )
                    .json(&PublishRequestBody::new(
                        source,
                        arguments.inner,
                        verifying_key.inner,
                    ))
                    .build()
                    .expect(zinc_const::panic::DATA_SERIALIZATION),
            )
            .map_err(Error::HttpRequest)?;

        if !http_response.status().is_success() {
            return Err(Error::ActionFailed(format!(
                "HTTP error ({}) {}",
                http_response.status(),
                http_response
                    .text()
                    .expect(zinc_const::panic::DATA_SERIALIZATION),
            )));
        }

        Ok(())
    }
}
