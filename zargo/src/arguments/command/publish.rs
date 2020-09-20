//!
//! The Zargo project manager `publish` subcommand.
//!

use std::convert::TryFrom;
use std::path::PathBuf;
use std::str::FromStr;

use colored::Colorize;
use failure::Fail;
use reqwest::Client as HttpClient;
use reqwest::Method;
use reqwest::Url;
use serde_json::Value as JsonValue;
use structopt::StructOpt;

use zinc_data::Network;
use zinc_data::PublishRequestBody;
use zinc_data::PublishRequestQuery;
use zinc_data::Source;
use zinc_data::SourceError;

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
use crate::file::bytecode::Bytecode as BytecodeFile;
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

    /// The contract instance name.
    #[structopt(long = "instance", help = "Sets the contract instance name")]
    pub instance: String,

    /// The network identifier, where the contract must be published to.
    #[structopt(
        long = "network",
        help = "Sets the network, which is either 'rinkeby', 'ropsten', or 'localhost'"
    )]
    pub network: String,
}

///
/// The Zargo project manager `publish` subcommand error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The invalid network error.
    #[fail(
        display = "network must be either `rinkeby`, `ropsten`, or `localhost`, but found `{}`",
        _0
    )]
    NetworkInvalid(String),
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
    /// The contract bytecode binary file error.
    #[fail(display = "bytecode binary file {}", _0)]
    BinaryFile(FileError),
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
        let network = Network::from_str(self.network.as_str()).map_err(Error::NetworkInvalid)?;

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

        let bytecode = BytecodeFile::try_from(&binary_path).map_err(Error::BinaryFile)?;

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

        let endpoint_url = format!(
            "{}{}",
            network.to_address(),
            zinc_const::zandbox::CONTRACT_PUBLISH_URL
        );

        eprintln!(
            "   {} the instance `{}` of `{} v{}` to network `{}`",
            "Uploading".bright_green(),
            self.instance,
            manifest.project.name,
            manifest.project.version,
            network,
        );
        let http_client = HttpClient::new();
        let mut http_response = http_client
            .execute(
                http_client
                    .request(
                        Method::POST,
                        Url::parse_with_params(
                            endpoint_url.as_str(),
                            PublishRequestQuery::new(
                                manifest.project.name,
                                manifest.project.version,
                                self.instance,
                                network,
                            ),
                        )
                        .expect(zinc_const::panic::DATA_CONVERSION),
                    )
                    .json(&PublishRequestBody::new(
                        source,
                        bytecode.inner,
                        arguments.inner,
                        verifying_key.inner,
                        "7726827caac94a7f9e1b160f7ea819f172f7b6f9d2a97f992c38edeab82d4110"
                            .to_owned(),
                    ))
                    .build()
                    .expect(zinc_const::panic::DATA_CONVERSION),
            )
            .map_err(Error::HttpRequest)?;

        if !http_response.status().is_success() {
            return Err(Error::ActionFailed(format!(
                "HTTP error ({}) {}",
                http_response.status(),
                http_response
                    .text()
                    .expect(zinc_const::panic::DATA_CONVERSION),
            )));
        }

        println!(
            "{}",
            serde_json::to_string_pretty(
                &http_response
                    .json::<JsonValue>()
                    .expect(zinc_const::panic::DATA_CONVERSION)
            )
            .expect(zinc_const::panic::DATA_CONVERSION)
        );

        Ok(())
    }
}
