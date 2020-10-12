//!
//! The Zargo package manager `publish` subcommand.
//!

use std::convert::TryFrom;
use std::path::PathBuf;
use std::str::FromStr;

use colored::Colorize;
use failure::Fail;
use reqwest::Client as HttpClient;
use reqwest::Method;
use reqwest::Url;
use structopt::StructOpt;

use zksync::web3::types::H256;
use zksync_types::tx::PackedEthSignature;

use zinc_data::InitializeRequestBody;
use zinc_data::InitializeRequestQuery;
use zinc_data::InitializeResponseBody;
use zinc_data::PublishRequestBody;
use zinc_data::PublishRequestQuery;
use zinc_data::PublishResponseBody;
use zinc_data::Source;
use zinc_data::SourceError;

use crate::arguments::command::IExecutable;
use crate::error::directory::Error as DirectoryError;
use crate::error::file::Error as FileError;
use crate::executable::compiler::Compiler;
use crate::executable::compiler::Error as CompilerError;
use crate::executable::virtual_machine::Error as VirtualMachineError;
use crate::executable::virtual_machine::VirtualMachine;
use crate::network::Network;
use crate::project::build::bytecode::Bytecode as BytecodeFile;
use crate::project::build::Directory as BuildDirectory;
use crate::project::data::arguments::Arguments as ArgumentsFile;
use crate::project::data::private_key::PrivateKey as PrivateKeyFile;
use crate::project::data::verifying_key::VerifyingKey as VerifyingKeyFile;
use crate::project::data::Directory as DataDirectory;
use crate::project::manifest::project_type::ProjectType;
use crate::project::manifest::Manifest as ManifestFile;
use crate::project::source::Directory as SourceDirectory;
use crate::transaction::error::Error as TransactionError;

///
/// The Zargo package manager `publish` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Uploads the smart contract to the specified network")]
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

    /// Sets the contract instance name.
    #[structopt(long = "instance")]
    pub instance: String,

    /// Sets the network name, where the contract must be published to.
    #[structopt(long = "network", default_value = "localhost")]
    pub network: String,
}

///
/// The Zargo package manager `publish` subcommand error.
///
#[derive(Debug, Fail)]
pub enum Error {
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
    /// The source code error.
    #[fail(display = "source code {}", _0)]
    Source(SourceError),
    /// The project binary build directory error.
    #[fail(display = "build directory {}", _0)]
    BuildDirectory(DirectoryError),
    /// The project template, keys, and other auxiliary data directory error.
    #[fail(display = "data directory {}", _0)]
    DataDirectory(DirectoryError),
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
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(self) -> Result<(), Self::Error> {
        let mut runtime = tokio::runtime::Runtime::new().expect(zinc_const::panic::ASYNC_RUNTIME);

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

        let source_directory_path = SourceDirectory::path(&manifest_path);
        let source = Source::try_from_path(&source_directory_path, true).map_err(Error::Source)?;

        DataDirectory::create(&manifest_path).map_err(Error::DataDirectory)?;
        let data_directory_path = DataDirectory::path(&manifest_path);
        let mut arguments_path = data_directory_path.clone();
        arguments_path.push(format!(
            "{}_{}.{}",
            zinc_const::file_name::WITNESS,
            zinc_const::contract::CONSTRUCTOR_NAME,
            zinc_const::extension::JSON,
        ));
        let mut proving_key_path = data_directory_path.clone();
        proving_key_path.push(zinc_const::file_name::PROVING_KEY);
        let mut verifying_key_path = data_directory_path.clone();
        verifying_key_path.push(zinc_const::file_name::VERIFYING_KEY.to_owned());
        let mut private_key_path = data_directory_path.clone();
        private_key_path.push(zinc_const::file_name::PRIVATE_KEY.to_owned());

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

        let arguments =
            ArgumentsFile::try_from_path(&arguments_path, zinc_const::contract::CONSTRUCTOR_NAME)
                .map_err(Error::ArgumentsFile)?;

        if !verifying_key_path.exists() {
            VirtualMachine::setup_contract(
                self.verbosity,
                &binary_path,
                zinc_const::contract::CONSTRUCTOR_NAME,
                &proving_key_path,
                &verifying_key_path,
            )
            .map_err(Error::VirtualMachine)?;
        }

        let verifying_key =
            VerifyingKeyFile::try_from(&verifying_key_path).map_err(Error::VerifyingKeyFile)?;

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
                            format!("{}{}", url, zinc_const::zandbox::CONTRACT_PUBLISH_URL)
                                .as_str(),
                            PublishRequestQuery::new(
                                manifest.project.name,
                                manifest.project.version,
                                self.instance,
                                network.into(),
                            ),
                        )
                        .expect(zinc_const::panic::DATA_CONVERSION),
                    )
                    .json(&PublishRequestBody::new(
                        source,
                        bytecode.inner,
                        arguments.inner,
                        verifying_key.inner,
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

        let response = http_response
            .json::<PublishResponseBody>()
            .expect(zinc_const::panic::DATA_CONVERSION);
        println!(
            "     {} {}",
            "Address".bright_green(),
            serde_json::to_string(&response.address)
                .expect(zinc_const::panic::DATA_CONVERSION)
                .replace("\"", "")
        );

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

        let initial_transfer = runtime
            .block_on(crate::transaction::new_initial(&wallet, response.address))
            .map_err(Error::Transaction)?;

        let mut http_response = http_client
            .execute(
                http_client
                    .request(
                        Method::PUT,
                        Url::parse_with_params(
                            format!("{}{}", url, zinc_const::zandbox::CONTRACT_INITIALIZE_URL)
                                .as_str(),
                            InitializeRequestQuery::new(response.address, network.into()),
                        )
                        .expect(zinc_const::panic::DATA_CONVERSION),
                    )
                    .json(&InitializeRequestBody::new(initial_transfer))
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
            .json::<InitializeResponseBody>()
            .expect(zinc_const::panic::DATA_CONVERSION);
        println!("  {} {}", "Account ID".bright_green(), response.account_id);

        Ok(())
    }
}
