//!
//! The Zargo package manager `publish` subcommand.
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
use crate::executable::compiler::Compiler;
use crate::executable::virtual_machine::VirtualMachine;
use crate::http::downloader::Downloader;
use crate::http::Client as HttpClient;
use crate::network::Network;
use crate::project::data::input::Input as InputFile;
use crate::project::data::private_key::PrivateKey as PrivateKeyFile;
use crate::project::data::verifying_key::VerifyingKey as VerifyingKeyFile;
use crate::project::data::Directory as DataDirectory;
use crate::project::src::Directory as SourceDirectory;
use crate::project::target::bytecode::Bytecode as BytecodeFile;
use crate::project::target::deps::Directory as TargetDependenciesDirectory;
use crate::project::target::Directory as TargetDirectory;

///
/// The Zargo package manager `publish` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Uploads the smart contract to the specified network")]
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

    /// Sets the contract instance name.
    #[structopt(long = "instance")]
    pub instance: String,

    /// Sets the network name, where the contract must be published to.
    #[structopt(long = "network", default_value = "localhost")]
    pub network: String,

    /// Sets the change-pubkey fee token.
    #[structopt(long = "change-pubkey-fee-token", default_value = "ETH")]
    pub change_pubkey_fee_token: String,
}

///
/// The publish data. Used for testing purposes.
///
pub struct Data {
    /// The address of the published contract instance.
    pub address: zksync_types::Address,
    /// The account ID of the published contract instance.
    pub account_id: zksync_types::AccountId,
}

impl Data {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(address: zksync_types::Address, account_id: zksync_types::AccountId) -> Self {
        Self {
            address,
            account_id,
        }
    }
}

impl Command {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        verbosity: usize,
        quiet: bool,
        manifest_path: PathBuf,
        instance: String,
        network: Option<String>,
        change_pubkey_fee_token: Option<String>,
    ) -> Self {
        Self {
            verbosity,
            quiet,
            manifest_path,
            instance,
            network: network
                .unwrap_or_else(|| Network::from(zksync::Network::Localhost).to_string()),
            change_pubkey_fee_token: change_pubkey_fee_token.unwrap_or_else(|| "ETH".to_owned()),
        }
    }

    ///
    /// Executes the command.
    ///
    pub async fn execute(self) -> anyhow::Result<Data> {
        let network = zksync::Network::from_str(self.network.as_str())
            .map(Network::from)
            .map_err(Error::NetworkInvalid)?;
        let url = network
            .try_into_url()
            .map_err(Error::NetworkUnimplemented)?;
        let http_client = HttpClient::new(url);

        let manifest = zinc_project::Manifest::try_from(&self.manifest_path)?;

        match manifest.project.r#type {
            zinc_project::ProjectType::Contract => {}
            _ => anyhow::bail!(Error::NotAContract),
        }

        let mut manifest_path = self.manifest_path;
        if manifest_path.is_file() {
            manifest_path.pop();
        }

        if let zinc_project::ProjectType::Contract = manifest.project.r#type {
            if !PrivateKeyFile::exists_at(&manifest_path) {
                PrivateKeyFile::default().write_to(&manifest_path)?;
            }
        }

        let source_directory_path = SourceDirectory::path(&manifest_path);
        let source =
            zinc_project::Source::try_from_path(&source_directory_path, &manifest_path, true)?;
        let project = zinc_project::Project::new(manifest.clone(), source);

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

        TargetDirectory::create(&manifest_path, true)?;
        let target_directory_path = TargetDirectory::path(&manifest_path, true);
        let mut binary_path = target_directory_path;
        binary_path.push(format!(
            "{}.{}",
            zinc_const::file_name::BINARY,
            zinc_const::extension::BINARY
        ));

        TargetDependenciesDirectory::create(&manifest_path)?;

        if let Some(dependencies) = manifest.dependencies {
            let network = zksync::Network::from_str(self.network.as_str())
                .map(Network::from)
                .map_err(Error::NetworkInvalid)?;
            let url = network
                .try_into_url()
                .map_err(Error::NetworkUnimplemented)?;
            let http_client = HttpClient::new(url);
            let mut downloader = Downloader::new(&http_client, &manifest_path);
            downloader.download_dependency_list(dependencies).await?;
        }

        Compiler::build_release(
            self.verbosity,
            self.quiet,
            manifest.project.name.as_str(),
            &manifest.project.version,
            &manifest_path,
            false,
        )?;

        let bytecode = BytecodeFile::try_from_path(&binary_path, true)?;

        let input = InputFile::try_from_path(&input_path)?;
        let arguments = input
            .inner
            .as_object()
            .ok_or_else(|| Error::MissingInputSection("arguments".to_owned()))?
            .get("arguments")
            .cloned()
            .ok_or_else(|| Error::MissingInputSection("arguments".to_owned()))?
            .as_object()
            .ok_or_else(|| Error::MissingInputSection("arguments".to_owned()))?
            .get(zinc_const::contract::CONSTRUCTOR_IDENTIFIER)
            .cloned()
            .ok_or_else(|| {
                Error::MissingInputSection(zinc_const::contract::CONSTRUCTOR_IDENTIFIER.to_owned())
            })?;

        if !verifying_key_path.exists() {
            VirtualMachine::setup_contract(
                self.verbosity,
                self.quiet,
                &binary_path,
                zinc_const::contract::CONSTRUCTOR_IDENTIFIER,
                &proving_key_path,
                &verifying_key_path,
            )?;
        }

        let verifying_key = VerifyingKeyFile::try_from(&verifying_key_path)?;

        if !self.quiet {
            eprintln!(
                "   {} the instance `{}` of `{} v{}` to network `{}`",
                "Uploading".bright_green(),
                self.instance,
                manifest.project.name,
                manifest.project.version,
                network,
            );
        }

        let response = http_client
            .publish(
                zinc_types::PublishRequestQuery::new(
                    manifest.project.name,
                    manifest.project.version,
                    self.instance,
                    self.change_pubkey_fee_token.clone(),
                ),
                zinc_types::PublishRequestBody::new(
                    project,
                    bytecode.inner,
                    arguments,
                    verifying_key.inner,
                ),
            )
            .await?;
        if !self.quiet {
            eprintln!(
                "     {} {}",
                "Address".bright_green(),
                serde_json::to_string(&response.address)
                    .expect(zinc_const::panic::DATA_CONVERSION)
                    .replace("\"", "")
            );
        }

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

        let initial_transfer = crate::transaction::new_initial(
            &wallet,
            response.address,
            self.change_pubkey_fee_token,
            response.change_pubkey_fee,
        )
        .await?;

        let address = response.address;
        let response = http_client
            .initialize(
                zinc_types::InitializeRequestQuery::new(response.address),
                zinc_types::InitializeRequestBody::new(initial_transfer),
            )
            .await?;
        if !self.quiet {
            eprintln!("  {} {}", "Account ID".bright_green(), response.account_id);
        }

        Ok(Data::new(address, response.account_id))
    }
}
