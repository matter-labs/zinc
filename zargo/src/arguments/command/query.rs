//!
//! The Zargo package manager `query` subcommand.
//!

use std::convert::TryFrom;
use std::path::PathBuf;
use std::str::FromStr;

use colored::Colorize;
use structopt::StructOpt;

use zinc_manifest::Manifest;
use zinc_manifest::ProjectType;

use crate::error::Error;
use crate::http::Client as HttpClient;
use crate::network::Network;
use crate::project::data::input::Input as InputFile;
use crate::project::data::Directory as DataDirectory;

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
    pub async fn execute(self) -> anyhow::Result<()> {
        let address = self.address["0x".len()..].parse()?;

        let network = zksync::Network::from_str(self.network.as_str())
            .map(Network::from)
            .map_err(Error::NetworkInvalid)?;
        let url = network
            .try_into_url()
            .map_err(Error::NetworkUnimplemented)?;
        let http_client = HttpClient::new(url);

        let manifest = Manifest::try_from(&self.manifest_path)?;

        match manifest.project.r#type {
            ProjectType::Contract => {}
            _ => anyhow::bail!(Error::NotAContract),
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

                let input = InputFile::try_from_path(&input_path)?;
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

        let response = http_client
            .query(
                zinc_zksync::QueryRequestQuery::new(address, self.method),
                zinc_zksync::QueryRequestBody::new(arguments),
            )
            .await?;
        println!(
            "{}",
            serde_json::to_string_pretty(&response).expect(zinc_const::panic::DATA_CONVERSION)
        );

        Ok(())
    }
}