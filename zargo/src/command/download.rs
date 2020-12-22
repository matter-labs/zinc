//!
//! The Zargo package manager `download` subcommand.
//!

use std::path::PathBuf;
use std::str::FromStr;

use structopt::StructOpt;

use crate::error::Error;
use crate::http::downloader::Downloader;
use crate::http::Client as HttpClient;
use crate::network::Network;

///
/// The Zargo package manager `download` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Initializes a new project in the specified directory")]
pub struct Command {
    /// Prints more logs, if passed several times.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbosity: usize,

    /// Suppresses output, if set.
    #[structopt(short = "q", long = "quiet")]
    pub quiet: bool,

    /// If set, shows the list of uploaded projects and exits.
    #[structopt(long = "list")]
    pub list: bool,

    /// Sets the project name to download.
    #[structopt(long = "name")]
    pub name: Option<String>,

    /// Sets the project version to download.
    #[structopt(long = "version")]
    pub version: Option<semver::Version>,

    /// Sets the network name, where the project must be downloaded from.
    #[structopt(long = "network", default_value = "localhost")]
    pub network: String,

    /// The path to the project directory to initialize.
    #[structopt(parse(from_os_str))]
    pub path: Option<PathBuf>,
}

impl Command {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        verbosity: usize,
        quiet: bool,
        list: bool,
        name: Option<String>,
        version: Option<semver::Version>,
        network: Option<String>,
        path: Option<PathBuf>,
    ) -> Self {
        Self {
            verbosity,
            quiet,
            list,
            name,
            version,
            network: network
                .unwrap_or_else(|| Network::from(zksync::Network::Localhost).to_string()),
            path,
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

        if self.list {
            for project in http_client.metadata().await?.projects.into_iter() {
                if !self.quiet {
                    println!("{}-{}", project.name, project.version);
                }
            }

            return Ok(());
        }

        let name = self.name.ok_or(Error::ProjectNameMissing)?;
        let version = self.version.ok_or(Error::ProjectVersionMissing)?;

        let project_path = match self.path {
            Some(path) => path,
            None => PathBuf::from(name.as_str()),
        };
        let mut downloader = Downloader::new(&http_client, &project_path);
        downloader.download_project(name, version).await?;

        Ok(())
    }
}
