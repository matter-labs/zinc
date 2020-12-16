//!
//! The Zandbox dependency downloader.
//!

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use async_recursion::async_recursion;
use colored::Colorize;

use crate::error::Error;
use crate::http::Client as HttpClient;

///
/// The Zandbox dependency downloader.
///
pub struct Downloader<'a> {
    /// The HTTP client reference.
    client: &'a HttpClient,
    /// The path to the directory where the dependencies must be downloaded to.
    directory: PathBuf,
    /// The downloaded dependencies set to prevent downloading the same project multiple times.
    downloads: HashSet<(String, semver::Version)>,
}

impl<'a> Downloader<'a> {
    /// The downloads hashmap default capacity.
    const DOWNLOADS_INITIAL_CAPACITY: usize = 64;

    ///
    /// A shortcut constructor.
    ///
    pub fn new(client: &'a HttpClient, directory: &PathBuf) -> Self {
        Self {
            client,
            directory: directory.to_owned(),
            downloads: HashSet::with_capacity(Self::DOWNLOADS_INITIAL_CAPACITY),
        }
    }

    ///
    /// Downloads a project.
    ///
    #[async_recursion]
    pub async fn download_project(
        &mut self,
        name: String,
        version: semver::Version,
    ) -> anyhow::Result<()> {
        eprintln!(" {} {} v{}", "Downloading".bright_green(), name, version);

        let project_path = self.directory.clone();
        if project_path.exists() {
            anyhow::bail!(Error::DirectoryAlreadyExists(
                project_path.as_os_str().to_owned()
            ));
        }

        let response = self
            .client
            .source(zinc_types::SourceRequestQuery::new(
                name.clone(),
                version.clone(),
            ))
            .await?;

        if response.zinc_version != env!("CARGO_PKG_VERSION") {
            anyhow::bail!(Error::CompilerVersionMismatch(
                format!("{}-{}", name, version),
                env!("CARGO_PKG_VERSION").to_string(),
                response.zinc_version,
            ));
        }

        fs::create_dir_all(&project_path)?;
        response.project.manifest.write_to(&project_path)?;
        response.project.source.write_to(&project_path)?;

        self.downloads.insert((name, version));
        if let Some(dependencies) = response.project.manifest.dependencies {
            self.download_dependency_list(dependencies).await?;
        }

        Ok(())
    }

    ///
    /// Downloads a dependency list.
    ///
    pub async fn download_dependency_list(
        &mut self,
        dependencies: HashMap<String, semver::Version>,
    ) -> anyhow::Result<()> {
        for (name, version) in dependencies.into_iter() {
            self.download_dependency(name, version).await?;
        }

        Ok(())
    }

    ///
    /// Downloads a dependency if it has not been downloaded yet.
    ///
    #[async_recursion]
    pub async fn download_dependency(
        &mut self,
        name: String,
        version: semver::Version,
    ) -> anyhow::Result<()> {
        if self.downloads.contains(&(name.clone(), version.clone())) {
            return Ok(());
        }

        let dependency_name = format!("{}-{}", name, version);
        let mut dependency_path = self.directory.clone();
        dependency_path.push(zinc_const::directory::TARGET_DEPS);
        dependency_path.push(dependency_name.as_str());
        if dependency_path.exists() {
            return Ok(());
        }

        eprintln!(" {} {} v{}", "Downloading".bright_green(), name, version);

        let response = self
            .client
            .source(zinc_types::SourceRequestQuery::new(
                name.clone(),
                version.clone(),
            ))
            .await?;

        if response.zinc_version != env!("CARGO_PKG_VERSION") {
            anyhow::bail!(Error::CompilerVersionMismatch(
                dependency_name,
                env!("CARGO_PKG_VERSION").to_string(),
                response.zinc_version,
            ));
        }

        fs::create_dir_all(&dependency_path)?;
        response.project.manifest.write_to(&dependency_path)?;
        response.project.source.write_to(&dependency_path)?;

        self.downloads.insert((name, version));
        if let Some(dependencies) = response.project.manifest.dependencies {
            self.download_dependency_list(dependencies).await?;
        }

        Ok(())
    }
}
