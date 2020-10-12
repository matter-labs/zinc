//!
//! The Zargo package manager `call` subcommand.
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

use zksync::web3::types::H256;
use zksync_types::tx::PackedEthSignature;

use zinc_data::CallRequestBody;
use zinc_data::CallRequestQuery;
use zinc_data::FeeRequestBody;
use zinc_data::FeeRequestQuery;
use zinc_data::FeeResponseBody;
use zinc_data::Transfer;

use crate::arguments::command::IExecutable;
use crate::error::file::Error as FileError;
use crate::network::Network;
use crate::project::data::arguments::Arguments as ArgumentsFile;
use crate::project::data::private_key::PrivateKey as PrivateKeyFile;
use crate::project::data::Directory as DataDirectory;
use crate::project::manifest::project_type::ProjectType;
use crate::project::manifest::Manifest as ManifestFile;
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

    /// Sets the contract method to call.
    #[structopt(long = "method")]
    pub method: String,

    /// Sets the path to the sender private key.
    #[structopt(
        long = "private-key",
        default_value = zinc_const::path::PRIVATE_KEY,
    )]
    pub private_key_path: PathBuf,
}

///
/// The Zargo package manager `call` subcommand error.
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
    /// The private key file error.
    #[fail(display = "private key file {}", _0)]
    PrivateKeyFile(FileError),
    /// The sender private key is invalid.
    #[fail(display = "sender private key is invalid: {}", _0)]
    SenderPrivateKeyInvalid(rustc_hex::FromHexError),
    /// The sender address cannot be derived from the private key.
    #[fail(
        display = "could not derive the ETH address from the private key: {}",
        _0
    )]
    SenderAddressDeriving(anyhow::Error),
    /// The wallet initialization error.
    #[fail(display = "wallet initialization: {}", _0)]
    WalletInitialization(zksync::error::ClientError),
    /// The transaction signing error.
    #[fail(display = "transaction: {}", _0)]
    Transaction(TransactionError),
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
        let mut runtime = tokio::runtime::Runtime::new().expect(zinc_const::panic::ASYNC_RUNTIME);

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
        let mut arguments_path = data_directory_path.clone();
        arguments_path.push(format!(
            "{}_{}.{}",
            zinc_const::file_name::WITNESS,
            self.method,
            zinc_const::extension::JSON,
        ));
        let mut private_key_path = data_directory_path;
        private_key_path.push(zinc_const::file_name::PRIVATE_KEY.to_owned());

        let arguments = ArgumentsFile::try_from_path(&arguments_path, self.method.as_str())
            .map_err(Error::ArgumentsFile)?;

        let private_key =
            PrivateKeyFile::try_from(&private_key_path).map_err(Error::PrivateKeyFile)?;

        let signer_private_key: H256 = private_key
            .inner
            .parse()
            .map_err(Error::SenderPrivateKeyInvalid)?;
        let signer_address = PackedEthSignature::address_from_private_key(&signer_private_key)
            .map_err(Error::SenderAddressDeriving)?;

        let wallet_credentials = runtime
            .block_on(zksync::WalletCredentials::from_eth_signer(
                signer_address,
                zksync_eth_signer::EthereumSigner::from_key(signer_private_key),
                network.into(),
            ))
            .expect(zinc_const::panic::DATA_CONVERSION);
        let wallet = runtime
            .block_on(zksync::Wallet::new(
                zksync::Provider::new(network.into()),
                wallet_credentials,
            ))
            .map_err(Error::WalletInitialization)?;

        let transfer = Transfer::try_from_json(&arguments.inner)
            .map_err(TransactionError::Parsing)
            .map_err(Error::Transaction)?;
        let transaction = runtime
            .block_on(crate::transaction::try_into_zksync(
                transfer.clone(),
                &wallet,
                None,
            ))
            .map_err(Error::Transaction)?;

        let http_client = HttpClient::new();
        let mut http_response = http_client
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
                    .json(&FeeRequestBody::new(arguments.inner.clone(), transaction))
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

        let response = http_response
            .json::<FeeResponseBody>()
            .expect(zinc_const::panic::DATA_CONVERSION);
        let contract_fee = response.fee;
        let transaction = runtime
            .block_on(crate::transaction::try_into_zksync(
                transfer,
                &wallet,
                Some(contract_fee),
            ))
            .map_err(Error::Transaction)?;

        let http_client = HttpClient::new();
        let mut http_response = http_client
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
                    .json(&CallRequestBody::new(arguments.inner, transaction))
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
