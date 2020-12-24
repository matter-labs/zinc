//!
//! The Zargo package manager `upload` subcommand.
//!

use std::convert::TryFrom;
use std::path::PathBuf;
use std::str::FromStr;

use colored::Colorize;
use structopt::StructOpt;

use crate::error::Error;
use crate::executable::compiler::Compiler;
use crate::executable::virtual_machine::VirtualMachine;
use crate::http::downloader::Downloader;
use crate::http::Client as HttpClient;
use crate::network::Network;
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

    /// Sets the network name, where the project must be uploaded to.
    #[structopt(long = "network", default_value = "localhost")]
    pub network: String,
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
    ) -> Self {
        Self {
            verbosity,
            quiet,
            manifest_path,
            network: network
                .unwrap_or_else(|| Network::from(zksync::Network::Localhost).to_string()),
        }
    }

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
        let http_client = HttpClient::new(url);

        let manifest = zinc_project::Manifest::try_from(&self.manifest_path)?;

        let mut manifest_path = self.manifest_path;
        if manifest_path.is_file() {
            manifest_path.pop();
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
        let mut verifying_key_path = data_directory_path;
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
                "   {} the {} `{} v{}` to network `{}`",
                "Uploading".bright_green(),
                manifest.project.r#type,
                manifest.project.name,
                manifest.project.version,
                network,
            );
        }

        http_client
            .upload(
                zinc_types::UploadRequestQuery::new(
                    manifest.project.name,
                    manifest.project.version,
                ),
                zinc_types::UploadRequestBody::new(project, bytecode.inner, verifying_key.inner),
            )
            .await?;

        Ok(())
    }
}
