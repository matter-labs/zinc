//!
//! The Zargo package manager `query` subcommand.
//!

pub mod error;

use std::convert::TryFrom;
use std::path::PathBuf;
use std::str::FromStr;

use colored::Colorize;
use reqwest::Client as HttpClient;
use reqwest::Method;
use reqwest::Url;
use serde_json::Value as JsonValue;
use structopt::StructOpt;

use zinc_manifest::Manifest;
use zinc_manifest::ProjectType;
use zinc_zksync::QueryRequestBody;
use zinc_zksync::QueryRequestQuery;

use crate::network::Network;
use crate::project::data::input::Input as InputFile;
use crate::project::data::Directory as DataDirectory;

use self::error::Error;

///
/// The Zargo package manager `query` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Queries a contract storage or calls an immutable method")]
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

impl Command {
    ///
    /// Executes the command.
    ///
    pub async fn execute(self) -> Result<(), Error> {
        let address = self.address["0x".len()..]
            .parse()
            .map_err(Error::InvalidContractAddress)?;

        let network = zksync::Network::from_str(self.network.as_str())
            .map(Network::from)
            .map_err(Error::NetworkInvalid)?;

        let url = network
            .try_into_url()
            .map_err(Error::NetworkUnimplemented)?;

        let manifest = Manifest::try_from(&self.manifest_path).map_err(Error::Manifest)?;

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
                let mut input_path = data_directory_path;
                input_path.push(format!(
                    "{}.{}",
                    zinc_const::file_name::INPUT,
                    zinc_const::extension::JSON,
                ));

                let input = InputFile::try_from_path(&input_path).map_err(Error::InputFile)?;
                let arguments = input
                    .inner
                    .as_object()
                    .ok_or(Error::InvalidInputData)?
                    .get("arguments")
                    .cloned()
                    .ok_or(Error::InvalidInputData)?
                    .as_object()
                    .ok_or(Error::InvalidInputData)?
                    .get(method)
                    .cloned()
                    .ok_or(Error::InvalidInputData)?;

                eprintln!(
                    "    {} method `{}` of the contract `{} v{}` with address {} on network `{}`",
                    "Querying".bright_green(),
                    method,
                    manifest.project.name,
                    manifest.project.version,
                    self.address,
                    network,
                );

                Some(arguments)
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
        let http_response = http_client
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
            .await
            .map_err(Error::HttpRequest)?;

        if !http_response.status().is_success() {
            return Err(Error::ActionFailed(format!(
                "HTTP error ({}) {}",
                http_response.status(),
                http_response
                    .text()
                    .await
                    .expect(zinc_const::panic::DATA_CONVERSION),
            )));
        }

        println!(
            "{}",
            serde_json::to_string_pretty(
                &http_response
                    .json::<JsonValue>()
                    .await
                    .expect(zinc_const::panic::DATA_CONVERSION)
            )
            .expect(zinc_const::panic::DATA_CONVERSION)
        );

        Ok(())
    }
}
