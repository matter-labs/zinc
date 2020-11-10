//!
//! The Zargo package manager `publish` subcommand.
//!

use std::convert::TryFrom;
use std::path::PathBuf;
use std::str::FromStr;

use colored::Colorize;
use num::BigUint;
use reqwest::Client as HttpClient;
use reqwest::Method;
use reqwest::Url;
use structopt::StructOpt;

use zksync::web3::types::H256;
use zksync_eth_signer::PrivateKeySigner;
use zksync_types::tx::PackedEthSignature;

use zinc_manifest::Manifest;
use zinc_manifest::ProjectType;
use zinc_source::Source;
use zinc_zksync::InitializeRequestBody;
use zinc_zksync::InitializeRequestQuery;
use zinc_zksync::InitializeResponseBody;
use zinc_zksync::PublishRequestBody;
use zinc_zksync::PublishRequestQuery;
use zinc_zksync::PublishResponseBody;

use crate::error::Error;
use crate::executable::compiler::Compiler;
use crate::executable::virtual_machine::VirtualMachine;
use crate::network::Network;
use crate::project::build::bytecode::Bytecode as BytecodeFile;
use crate::project::build::Directory as BuildDirectory;
use crate::project::data::input::Input as InputFile;
use crate::project::data::private_key::PrivateKey as PrivateKeyFile;
use crate::project::data::verifying_key::VerifyingKey as VerifyingKeyFile;
use crate::project::data::Directory as DataDirectory;
use crate::project::source::Directory as SourceDirectory;

///
/// The Zargo package manager `publish` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Uploads the smart contract to the specified network")]
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

    /// Sets the contract instance name.
    #[structopt(long = "instance")]
    pub instance: String,

    /// Sets the network name, where the contract must be published to.
    #[structopt(long = "network", default_value = "localhost")]
    pub network: String,

    /// Sets the initial deposit token.
    #[structopt(long = "deposit-token", default_value = "ETH")]
    pub deposit_token: String,

    /// Sets the initial deposit amount.
    #[structopt(long = "deposit-amount", default_value = "0")]
    pub deposit_amount: String,
}

impl Command {
    ///
    /// Executes the command.
    ///
    pub async fn execute(self) -> anyhow::Result<()> {
        let network = zksync::Network::from_str(self.network.as_str())
            .map(Network::from)
            .map_err(Error::NetworkInvalid)?;

        let url = network
            .try_into_url()
            .map_err(Error::NetworkUnimplemented)?;

        let manifest = Manifest::try_from(&self.manifest_path)?;

        match manifest.project.r#type {
            ProjectType::Contract => {}
            _ => anyhow::bail!(Error::NotAContract),
        }

        let mut manifest_path = self.manifest_path;
        if manifest_path.is_file() {
            manifest_path.pop();
        }

        let source_directory_path = SourceDirectory::path(&manifest_path);
        let source = Source::try_from_path(&source_directory_path, true)?;

        DataDirectory::create(&manifest_path)?;
        let data_directory_path = DataDirectory::path(&manifest_path);
        let mut input_path = data_directory_path.clone();
        input_path.push(format!(
            "{}.{}",
            zinc_const::file_name::INPUT,
            zinc_const::extension::JSON,
        ));
        let mut proving_key_path = data_directory_path.clone();
        proving_key_path.push(zinc_const::file_name::PROVING_KEY);
        let mut verifying_key_path = data_directory_path.clone();
        verifying_key_path.push(zinc_const::file_name::VERIFYING_KEY.to_owned());
        let mut private_key_path = data_directory_path.clone();
        private_key_path.push(zinc_const::file_name::PRIVATE_KEY.to_owned());

        BuildDirectory::create(&manifest_path)?;
        let build_directory_path = BuildDirectory::path(&manifest_path);
        let mut binary_path = build_directory_path;
        binary_path.push(format!(
            "{}.{}",
            zinc_const::file_name::BINARY,
            zinc_const::extension::BINARY
        ));

        if let ProjectType::Contract = manifest.project.r#type {
            if !PrivateKeyFile::exists_at(&data_directory_path) {
                PrivateKeyFile::default().write_to(&data_directory_path)?;
            }
        }

        Compiler::build_release(
            self.verbosity,
            manifest.project.name.as_str(),
            manifest.project.version.as_str(),
            &manifest_path,
            &data_directory_path,
            &source_directory_path,
            &binary_path,
            false,
        )?;

        let bytecode = BytecodeFile::try_from(&binary_path)?;

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
            .get(zinc_const::contract::CONSTRUCTOR_NAME)
            .cloned()
            .ok_or(Error::ConstructorArgumentsNotFound)?;

        if !verifying_key_path.exists() {
            VirtualMachine::setup_contract(
                self.verbosity,
                &binary_path,
                zinc_const::contract::CONSTRUCTOR_NAME,
                &proving_key_path,
                &verifying_key_path,
            )?;
        }

        let verifying_key = VerifyingKeyFile::try_from(&verifying_key_path)?;

        eprintln!(
            "   {} the instance `{}` of `{} v{}` to network `{}`",
            "Uploading".bright_green(),
            self.instance,
            manifest.project.name,
            manifest.project.version,
            network,
        );

        let http_client = HttpClient::new();

        let http_response = http_client
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
                        arguments,
                        verifying_key.inner,
                    ))
                    .build()
                    .expect(zinc_const::panic::DATA_CONVERSION),
            )
            .await?;

        if !http_response.status().is_success() {
            anyhow::bail!(Error::ActionFailed(format!(
                "HTTP error ({}) {}",
                http_response.status(),
                http_response
                    .text()
                    .await
                    .expect(zinc_const::panic::DATA_CONVERSION),
            )));
        }

        let response = http_response
            .json::<PublishResponseBody>()
            .await
            .expect(zinc_const::panic::DATA_CONVERSION);
        println!(
            "     {} {}",
            "Address".bright_green(),
            serde_json::to_string(&response.address)
                .expect(zinc_const::panic::DATA_CONVERSION)
                .replace("\"", "")
        );

        let private_key = PrivateKeyFile::try_from(&private_key_path)?;

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
            zksync::Wallet::new(zksync::Provider::new(network.into()), wallet_credentials).await?;

        let initial_deposit_amount: BigUint =
            zinc_math::bigint_from_str(self.deposit_amount.as_str())?
                .to_biguint()
                .expect(zinc_const::panic::DATA_CONVERSION);
        let initial_transfer = crate::transaction::new_initial(
            &wallet,
            response.address,
            self.deposit_token,
            initial_deposit_amount,
        )
        .await?;

        let http_response = http_client
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
            .await?;

        if !http_response.status().is_success() {
            anyhow::bail!(Error::ActionFailed(format!(
                "HTTP error ({}) {}",
                http_response.status(),
                http_response
                    .text()
                    .await
                    .expect(zinc_const::panic::DATA_CONVERSION),
            )));
        }

        let response = http_response
            .json::<InitializeResponseBody>()
            .await
            .expect(zinc_const::panic::DATA_CONVERSION);
        println!("  {} {}", "Account ID".bright_green(), response.account_id);

        Ok(())
    }
}
