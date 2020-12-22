//!
//! The Zargo package manager `call` subcommand.
//!

use std::convert::TryFrom;
use std::path::PathBuf;
use std::str::FromStr;

use colored::Colorize;
use structopt::StructOpt;

use zksync::web3::types::H256;
use zksync_eth_signer::PrivateKeySigner;
use zksync_types::tx::PackedEthSignature;

use crate::error::Error;
use crate::http::Client as HttpClient;
use crate::network::Network;
use crate::project::data::input::Input as InputFile;
use crate::project::data::private_key::PrivateKey as PrivateKeyFile;
use crate::project::data::Directory as DataDirectory;
use crate::transaction::error::Error as TransactionError;

///
/// The Zargo package manager `call` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Calls a mutable smart contract method")]
pub struct Command {
    /// Prints more logs, if passed several times.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbosity: usize,

    /// Suppresses output, if set.
    #[structopt(short = "q", long = "quiet")]
    pub quiet: bool,

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

    /// Sets the contract method to call.
    #[structopt(long = "method")]
    pub method: String,

    /// Sets the path to the sender private key.
    #[structopt(long = "private-key", default_value = "./data/private_key")]
    pub private_key_path: PathBuf,
}

impl Command {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        verbosity: usize,
        quiet: bool,
        manifest_path: PathBuf,
        network: Option<String>,
        address: String,
        method: String,
    ) -> Self {
        Self {
            verbosity,
            quiet,
            manifest_path,
            network: network
                .unwrap_or_else(|| Network::from(zksync::Network::Localhost).to_string()),
            address,
            method,
            private_key_path: PathBuf::from("./data/private_key"),
        }
    }

    ///
    /// Executes the command.
    ///
    pub async fn execute(self) -> anyhow::Result<serde_json::Value> {
        let address = self.address["0x".len()..].parse()?;

        let network = zksync::Network::from_str(self.network.as_str())
            .map(Network::from)
            .map_err(Error::NetworkInvalid)?;
        let url = network
            .try_into_url()
            .map_err(Error::NetworkUnimplemented)?;
        let http_client = HttpClient::new(url);

        let manifest = zinc_project::Manifest::try_from(&self.manifest_path)?;

        if !self.quiet {
            eprintln!(
                "     {} method `{}` of the contract `{} v{}` with address {} on network `{}`",
                "Calling".bright_green(),
                self.method,
                manifest.project.name,
                manifest.project.version,
                self.address,
                network,
            );
        }

        match manifest.project.r#type {
            zinc_project::ProjectType::Contract => {}
            _ => anyhow::bail!(Error::NotAContract),
        }

        let mut manifest_path = self.manifest_path;
        if manifest_path.is_file() {
            manifest_path.pop();
        }

        let data_directory_path = DataDirectory::path(&manifest_path);
        let mut input_path = data_directory_path.clone();
        input_path.push(format!(
            "{}.{}",
            zinc_const::file_name::INPUT,
            zinc_const::extension::JSON,
        ));

        let input = InputFile::try_from_path(&input_path)?;
        let method = self.method;
        let arguments = input
            .inner
            .as_object()
            .ok_or_else(|| Error::MissingInputSection("arguments".to_owned()))?
            .get("arguments")
            .cloned()
            .ok_or_else(|| Error::MissingInputSection("arguments".to_owned()))?
            .as_object()
            .ok_or_else(|| Error::MissingInputSection("arguments".to_owned()))?
            .get(method.as_str())
            .cloned()
            .ok_or_else(|| Error::MissingInputSection(format!("arguments.{}", method)))?;

        let private_key = PrivateKeyFile::try_from(&manifest_path)?;

        let signer_private_key: H256 = private_key.inner.parse()?;
        let signer_address = PackedEthSignature::address_from_private_key(&signer_private_key)?;

        let wallet_credentials = zksync::WalletCredentials::from_eth_signer(
            signer_address,
            PrivateKeySigner::new(signer_private_key),
            network.into(),
        )
        .await
        .expect(zinc_const::panic::DATA_CONVERSION);
        let wallet =
            zksync::Wallet::new(zksync::RpcProvider::new(network.into()), wallet_credentials)
                .await?;

        let msg = input
            .inner
            .as_object()
            .ok_or_else(|| {
                Error::MissingInputSection(
                    zinc_const::contract::TRANSACTION_VARIABLE_NAME.to_owned(),
                )
            })?
            .get(zinc_const::contract::TRANSACTION_VARIABLE_NAME)
            .cloned()
            .ok_or_else(|| {
                Error::MissingInputSection(
                    zinc_const::contract::TRANSACTION_VARIABLE_NAME.to_owned(),
                )
            })?;
        let msg = zinc_types::TransactionMsg::try_from(&msg).map_err(TransactionError::Parsing)?;
        let transaction = crate::transaction::try_into_zksync(msg.clone(), &wallet, None).await?;

        let response = http_client
            .fee(
                zinc_types::FeeRequestQuery::new(address, method.clone()),
                zinc_types::FeeRequestBody::new(arguments.clone(), transaction),
            )
            .await?;
        let contract_fee = response.fee;
        let transaction = crate::transaction::try_into_zksync(
            msg,
            &wallet,
            Some(zinc_types::num_compat_forward(contract_fee)),
        )
        .await?;

        let response = http_client
            .call(
                zinc_types::CallRequestQuery::new(address, method),
                zinc_types::CallRequestBody::new(arguments, transaction),
            )
            .await?;
        if !self.quiet {
            println!(
                "{}",
                serde_json::to_string_pretty(&response).expect(zinc_const::panic::DATA_CONVERSION)
            );
        }

        Ok(response)
    }
}
