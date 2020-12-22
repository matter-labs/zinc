//!
//! The Zargo package manager `run` subcommand.
//!

use std::convert::TryFrom;
use std::path::PathBuf;
use std::str::FromStr;

use structopt::StructOpt;

use crate::error::Error;
use crate::executable::compiler::Compiler;
use crate::executable::virtual_machine::VirtualMachine;
use crate::http::downloader::Downloader;
use crate::http::Client as HttpClient;
use crate::network::Network;
use crate::project::data::private_key::PrivateKey as PrivateKeyFile;
use crate::project::data::Directory as DataDirectory;
use crate::project::target::deps::Directory as TargetDependenciesDirectory;
use crate::project::target::Directory as TargetDirectory;

///
/// The Zargo package manager `run` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Runs the project and prints its output")]
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

    /// The contract method to call. Only for contracts.
    #[structopt(long = "method")]
    pub method: Option<String>,

    /// Uses the release build.
    #[structopt(long = "release")]
    pub is_release: bool,

    /// Sets the network name, where the contract must be published to.
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
        method: Option<String>,
        is_release: bool,
        network: Option<String>,
    ) -> Self {
        Self {
            verbosity,
            quiet,
            manifest_path,
            method,
            is_release,
            network: network
                .unwrap_or_else(|| Network::from(zksync::Network::Localhost).to_string()),
        }
    }

    ///
    /// Executes the command.
    ///
    pub async fn execute(self) -> anyhow::Result<()> {
        let manifest = zinc_project::Manifest::try_from(&self.manifest_path)?;

        match manifest.project.r#type {
            zinc_project::ProjectType::Contract if self.method.is_none() => {
                anyhow::bail!(Error::MethodMissing)
            }
            _ => {}
        }

        let mut manifest_path = self.manifest_path.clone();
        if manifest_path.is_file() {
            manifest_path.pop();
        }

        if self.method.is_some() && !PrivateKeyFile::exists_at(&manifest_path) {
            PrivateKeyFile::default().write_to(&manifest_path)?;
        }

        TargetDirectory::create(&manifest_path, self.is_release)?;
        let target_directory_path = TargetDirectory::path(&manifest_path, self.is_release);
        let mut binary_path = target_directory_path;
        binary_path.push(format!(
            "{}.{}",
            zinc_const::file_name::BINARY,
            zinc_const::extension::BINARY
        ));

        TargetDependenciesDirectory::create(&manifest_path)?;

        DataDirectory::create(&manifest_path)?;
        let data_directory_path = DataDirectory::path(&manifest_path);
        let mut input_path = data_directory_path.clone();
        input_path.push(format!(
            "{}.{}",
            zinc_const::file_name::INPUT,
            zinc_const::extension::JSON,
        ));
        let mut output_path = data_directory_path;
        output_path.push(format!(
            "{}.{}",
            zinc_const::file_name::OUTPUT,
            zinc_const::extension::JSON,
        ));

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

        if self.is_release {
            Compiler::build_release(
                self.verbosity,
                self.quiet,
                manifest.project.name.as_str(),
                &manifest.project.version,
                &manifest_path,
                false,
            )?;
        } else {
            Compiler::build_debug(
                self.verbosity,
                self.quiet,
                manifest.project.name.as_str(),
                &manifest.project.version,
                &manifest_path,
                false,
            )?;
        }

        match self.method {
            Some(method) => VirtualMachine::run_contract(
                self.verbosity,
                self.quiet,
                &binary_path,
                &input_path,
                &output_path,
                method.as_str(),
            ),
            None => VirtualMachine::run_circuit(
                self.verbosity,
                self.quiet,
                &binary_path,
                &input_path,
                &output_path,
            ),
        }?;

        Ok(())
    }
}
