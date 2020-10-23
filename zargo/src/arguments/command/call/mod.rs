//!
//! The Zargo package manager `call` subcommand.
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

use zksync::web3::types::H256;
use zksync_types::tx::PackedEthSignature;

use zinc_manifest::Manifest;
use zinc_manifest::ProjectType;
use zinc_zksync::CallRequestBody;
use zinc_zksync::CallRequestQuery;
use zinc_zksync::FeeRequestBody;
use zinc_zksync::FeeRequestQuery;
use zinc_zksync::FeeResponseBody;
use zinc_zksync::TransactionMsg;

use crate::network::Network;
use crate::project::data::input::Input as InputFile;
use crate::project::data::private_key::PrivateKey as PrivateKeyFile;
use crate::project::data::Directory as DataDirectory;
use crate::transaction::error::Error as TransactionError;

use self::error::Error;

///
/// The Zargo package manager `call` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Calls a mutable smart contract method")]
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

    /// Sets the contract method to call.
    #[structopt(long = "method")]
    pub method: String,

    /// Sets the path to the sender private key.
    #[structopt(long = "private-key", default_value = "./data/private_key")]
    pub private_key_path: PathBuf,
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

        eprintln!(
            "     {} method `{}` of the contract `{} v{}` with address {} on network `{}`",
            "Calling".bright_green(),
            self.method,
            manifest.project.name,
            manifest.project.version,
            self.address,
            network,
        );

        match manifest.project.r#type {
            ProjectType::Contract => {}
            _ => return Err(Error::NotAContract),
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
        let mut private_key_path = data_directory_path;
        private_key_path.push(zinc_const::file_name::PRIVATE_KEY.to_owned());

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
            .get(self.method.as_str())
            .cloned()
            .ok_or(Error::InvalidInputData)?;

        let private_key =
            PrivateKeyFile::try_from(&private_key_path).map_err(Error::PrivateKeyFile)?;

        let signer_private_key: H256 = private_key
            .inner
            .parse()
            .map_err(Error::SenderPrivateKeyInvalid)?;
        let signer_address = PackedEthSignature::address_from_private_key(&signer_private_key)
            .map_err(Error::SenderAddressDeriving)?;

        let wallet_credentials = zksync::WalletCredentials::from_eth_signer(
            signer_address,
            zksync_eth_signer::EthereumSigner::from_key(signer_private_key),
            network.into(),
        )
        .await
        .expect(zinc_const::panic::DATA_CONVERSION);
        let wallet = zksync::Wallet::new(zksync::Provider::new(network.into()), wallet_credentials)
            .await
            .map_err(Error::WalletInitialization)?;

        let msg = input
            .inner
            .as_object()
            .ok_or(Error::InvalidInputData)?
            .get("msg")
            .cloned()
            .ok_or(Error::InvalidInputData)?;
        let msg = TransactionMsg::try_from(&msg)
            .map_err(TransactionError::Parsing)
            .map_err(Error::Transaction)?;
        let transaction = crate::transaction::try_into_zksync(msg.clone(), &wallet, None)
            .await
            .map_err(Error::Transaction)?;

        let http_client = HttpClient::new();
        let http_response = http_client
            .execute(
                http_client
                    .request(
                        Method::PUT,
                        Url::parse_with_params(
                            format!("{}{}", url, zinc_const::zandbox::CONTRACT_FEE_URL).as_str(),
                            FeeRequestQuery::new(address, self.method.clone(), network.into()),
                        )
                        .expect(zinc_const::panic::DATA_CONVERSION),
                    )
                    .json(&FeeRequestBody::new(arguments.clone(), transaction))
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

        let response = http_response
            .json::<FeeResponseBody>()
            .await
            .expect(zinc_const::panic::DATA_CONVERSION);
        let contract_fee = response.fee;
        let transaction = crate::transaction::try_into_zksync(
            msg,
            &wallet,
            Some(zinc_zksync::num_compat_forward(contract_fee)),
        )
        .await
        .map_err(Error::Transaction)?;

        let http_client = HttpClient::new();
        let http_response = http_client
            .execute(
                http_client
                    .request(
                        Method::POST,
                        Url::parse_with_params(
                            format!("{}{}", url, zinc_const::zandbox::CONTRACT_CALL_URL).as_str(),
                            CallRequestQuery::new(address, self.method, network.into()),
                        )
                        .expect(zinc_const::panic::DATA_CONVERSION),
                    )
                    .json(&CallRequestBody::new(arguments, transaction))
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
