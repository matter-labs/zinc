//!
//! The Zargo project manager `query` subcommand.
//!

use std::convert::TryFrom;
use std::fs;
use std::io;
use std::io::Read;
use std::path::PathBuf;

use colored::Colorize;
use failure::Fail;
use reqwest::Client as HttpClient;
use reqwest::Method;
use reqwest::Url;
use serde_json::Value as JsonValue;
use structopt::StructOpt;

use zinc_source::QueryRequestBody;
use zinc_source::QueryRequestQuery;

use crate::arguments::command::IExecutable;
use crate::directory::data::Directory as DataDirectory;
use crate::manifest::project_type::ProjectType;
use crate::manifest::Error as ManifestError;
use crate::manifest::Manifest;

///
/// The Zargo project manager `query` subcommand.
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

    /// The contract method to call.
    #[structopt(long = "method", help = "The contract method to call")]
    pub method: String,
}

///
/// The Zargo project manager `query` subcommand error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The manifest file error.
    #[fail(display = "manifest file {}", _0)]
    ManifestFile(ManifestError),
    /// The project is not a contract.
    #[fail(display = "not a contract")]
    NotAContract,
    /// The contract method arguments file error.
    #[fail(display = "arguments file {}", _0)]
    ArgumentsFile(FileError),
    /// The publish HTTP request error.
    #[fail(display = "HTTP request: {}", _0)]
    HttpRequest(reqwest::Error),
    /// The smart contract server failure.
    #[fail(display = "action failed: {}", _0)]
    ActionFailed(String),
}

///
/// The arguments file error. TODO: move to a single file and add the path
///
#[derive(Debug, Fail)]
pub enum FileError {
    /// File opening error.
    #[fail(display = "opening: {}", _0)]
    Opening(io::Error),
    /// File metadata getting error.
    #[fail(display = "metadata: {}", _0)]
    Metadata(io::Error),
    /// File reading error.
    #[fail(display = "reading: {}", _0)]
    Reading(io::Error),
    /// File contents parsing error.
    #[fail(display = "parsing: {}", _0)]
    Parsing(serde_json::Error),
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(self) -> Result<(), Self::Error> {
        let manifest = Manifest::try_from(&self.manifest_path).map_err(Error::ManifestFile)?;

        match manifest.project.r#type {
            ProjectType::Contract => {}
            _ => return Err(Error::NotAContract),
        }

        let mut manifest_path = self.manifest_path;
        if manifest_path.is_file() {
            manifest_path.pop();
        }

        let data_directory_path = DataDirectory::path(&manifest_path);
        let mut arguments_path = data_directory_path;
        arguments_path.push(format!(
            "{}_{}.{}",
            zinc_const::file_name::WITNESS,
            self.method,
            zinc_const::extension::JSON,
        ));

        let mut arguments_file = fs::File::open(arguments_path)
            .map_err(FileError::Opening)
            .map_err(Error::ArgumentsFile)?;
        let arguments_file_size = arguments_file
            .metadata()
            .map_err(FileError::Metadata)
            .map_err(Error::ArgumentsFile)?
            .len() as usize;
        let mut arguments_file_str = String::with_capacity(arguments_file_size);
        arguments_file
            .read_to_string(&mut arguments_file_str)
            .map_err(FileError::Reading)
            .map_err(Error::ArgumentsFile)?;
        let arguments: JsonValue = serde_json::from_str(arguments_file_str.as_str())
            .map_err(FileError::Parsing)
            .map_err(Error::ArgumentsFile)?;

        eprintln!(
            "    {} method `{}` of the contract `{} v{} with ID {}`",
            "Querying".bright_green(),
            self.method,
            manifest.project.name,
            manifest.project.version,
            self.contract_id,
        );
        let http_client = HttpClient::new();
        let mut http_response = http_client
            .execute(
                http_client
                    .request(
                        Method::PUT,
                        Url::parse_with_params(
                            zinc_const::zandbox::CONTRACT_QUERY_URL,
                            QueryRequestQuery::new(self.contract_id, self.method).into_vec(),
                        )
                        .expect(zinc_const::panic::DATA_SERIALIZATION),
                    )
                    .json(&QueryRequestBody::new(arguments))
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

        println!(
            "{}",
            http_response
                .json::<JsonValue>()
                .expect(zinc_const::panic::DATA_SERIALIZATION)
        );

        Ok(())
    }
}
