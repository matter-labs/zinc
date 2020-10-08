//!
//! The Zargo package manager `query` subcommand.
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

use zinc_data::QueryRequestBody;
use zinc_data::QueryRequestQuery;

use crate::arguments::command::IExecutable;
use crate::error::file::Error as FileError;
use crate::network::Network;
use crate::project::data::arguments::Arguments as ArgumentsFile;
use crate::project::data::Directory as DataDirectory;
use crate::project::manifest::project_type::ProjectType;
use crate::project::manifest::Manifest as ManifestFile;

///
/// The Zargo package manager `query` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Queries a contract storage or calls an immutable method")]
pub struct Command {
    /// Prints more logs, if passed several times.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbosity: usize,

    /// The path to the Zargo project manifest file.
    #[structopt(
        long = "manifest-path",
        parse(from_os_str),
        default_value = zinc_const::path::MANIFEST,
    )]
    pub manifest_path: PathBuf,

    /// Sets the network name, where the contract resides.
    #[structopt(long = "network", default_value = "localhost")]
    pub network: String,

    /// Sets the ETH address of the contract.
    #[structopt(long = "address")]
    pub address: String,

    /// Sets the contract method to call. If not specified, the contract storage is queried.
    #[structopt(long = "method")]
    pub method: Option<String>,
}

///
/// The Zargo package manager `query` subcommand error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The ETH address is invalid.
    #[fail(display = "invalid ETH address: {}", _0)]
    InvalidContractAddress(rustc_hex::FromHexError),
    /// The invalid network error.
    #[fail(display = "invalid network name: {}", _0)]
    NetworkInvalid(String),
    /// The unimplemented network error.
    #[fail(display = "unimplemented network: {}", _0)]
    NetworkUnimplemented(zksync::Network),
    /// The manifest file error.
    #[fail(display = "manifest file {}", _0)]
    ManifestFile(FileError<toml::de::Error>),
    /// The project is not a contract.
    #[fail(display = "not a contract")]
    NotAContract,
    /// The contract method arguments file error.
    #[fail(display = "arguments file {}", _0)]
    ArgumentsFile(FileError<serde_json::Error>),
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
        let address = self.address["0x".len()..]
            .parse()
            .map_err(Error::InvalidContractAddress)?;

        let network = zksync::Network::from_str(self.network.as_str())
            .map(Network::from)
            .map_err(Error::NetworkInvalid)?;

        let url = network
            .try_into_url()
            .map_err(Error::NetworkUnimplemented)?;

        let manifest = ManifestFile::try_from(&self.manifest_path).map_err(Error::ManifestFile)?;

        match manifest.project.r#type {
            ProjectType::Contract => {}
            _ => return Err(Error::NotAContract),
        }

        let mut manifest_path = self.manifest_path;
        if manifest_path.is_file() {
            manifest_path.pop();
        }

        let arguments = match self.method {
            Some(ref method) => {
                let data_directory_path = DataDirectory::path(&manifest_path);
                let mut arguments_path = data_directory_path;
                arguments_path.push(format!(
                    "{}_{}.{}",
                    zinc_const::file_name::WITNESS,
                    method,
                    zinc_const::extension::JSON,
                ));

                let arguments = ArgumentsFile::try_from_path(&arguments_path, method.as_str())
                    .map_err(Error::ArgumentsFile)?;

                eprintln!(
                    "    {} method `{}` of the contract `{} v{}` with address {} on network `{}`",
                    "Querying".bright_green(),
                    method,
                    manifest.project.name,
                    manifest.project.version,
                    self.address,
                    network,
                );

                Some(arguments.inner)
            }
            None => {
                eprintln!(
                    "    {} the storage of the contract `{} v{}` with address {} on network `{}`",
                    "Querying".bright_green(),
                    manifest.project.name,
                    manifest.project.version,
                    self.address,
                    network,
                );

                None
            }
        };

        let http_client = HttpClient::new();
        let mut http_response = http_client
            .execute(
                http_client
                    .request(
                        Method::PUT,
                        Url::parse_with_params(
                            format!("{}{}", url, zinc_const::zandbox::CONTRACT_QUERY_URL).as_str(),
                            QueryRequestQuery::new(address, self.method, network.into()),
                        )
                        .expect(zinc_const::panic::DATA_CONVERSION),
                    )
                    .json(&QueryRequestBody::new(arguments))
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
