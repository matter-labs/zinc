//!
//! The Zargo package manager `test` subcommand.
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
use crate::project::target::deps::Directory as TargetDependenciesDirectory;
use crate::project::target::Directory as TargetDirectory;

///
/// The Zargo package manager `test` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Runs the project unit tests")]
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
        let manifest = zinc_project::Manifest::try_from(&self.manifest_path)?;

        let mut manifest_path = self.manifest_path.clone();
        if manifest_path.is_file() {
            manifest_path.pop();
        }

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
            true,
        )?;

        VirtualMachine::test(self.verbosity, self.quiet, &binary_path)?;

        Ok(())
    }
}
