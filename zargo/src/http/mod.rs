//!
//! The Zandbox HTTP client.
//!

pub mod downloader;

use reqwest::Method;
use reqwest::Url;

use crate::error::Error;

///
/// The Zandbox HTTP client.
///
pub struct Client {
    /// The inner HTTP client.
    inner: reqwest::Client,
    /// The Zandbox URL.
    url: String,
}

impl Client {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(url: String) -> Self {
        Self {
            inner: reqwest::Client::new(),
            url,
        }
    }

    ///
    /// Downloads projects metadata from the Zandbox server.
    ///
    pub async fn metadata(&self) -> anyhow::Result<zinc_types::MetadataResponseBody> {
        let response = self
            .inner
            .execute(
                self.inner
                    .request(
                        Method::GET,
                        Url::parse(
                            format!("{}{}", self.url, zinc_const::zandbox::PROJECT_URL).as_str(),
                        )
                        .expect(zinc_const::panic::DATA_CONVERSION),
                    )
                    .build()
                    .expect(zinc_const::panic::DATA_CONVERSION),
            )
            .await?;

        if !response.status().is_success() {
            anyhow::bail!(Error::ProjectMetadata(format!(
                "HTTP error ({}) {}",
                response.status(),
                response
                    .text()
                    .await
                    .expect(zinc_const::panic::DATA_CONVERSION),
            )));
        }

        Ok(response
            .json::<zinc_types::MetadataResponseBody>()
            .await
            .expect(zinc_const::panic::DATA_CONVERSION))
    }

    ///
    /// Uploads a project to the Zandbox server.
    ///
    pub async fn upload(
        &self,
        query: zinc_types::UploadRequestQuery,
        body: zinc_types::UploadRequestBody,
    ) -> anyhow::Result<()> {
        let response = self
            .inner
            .execute(
                self.inner
                    .request(
                        Method::POST,
                        Url::parse_with_params(
                            format!("{}{}", self.url, zinc_const::zandbox::PROJECT_URL).as_str(),
                            query,
                        )
                        .expect(zinc_const::panic::DATA_CONVERSION),
                    )
                    .json(&body)
                    .build()
                    .expect(zinc_const::panic::DATA_CONVERSION),
            )
            .await?;

        if !response.status().is_success() {
            anyhow::bail!(Error::ProjectUploading(format!(
                "HTTP error ({}) {}",
                response.status(),
                response
                    .text()
                    .await
                    .expect(zinc_const::panic::DATA_CONVERSION),
            )));
        }

        Ok(())
    }

    ///
    /// Publishes a contract to the Zandbox server.
    ///
    pub async fn publish(
        &self,
        query: zinc_types::PublishRequestQuery,
        body: zinc_types::PublishRequestBody,
    ) -> anyhow::Result<zinc_types::PublishResponseBody> {
        let response = self
            .inner
            .execute(
                self.inner
                    .request(
                        Method::POST,
                        Url::parse_with_params(
                            format!("{}{}", self.url, zinc_const::zandbox::CONTRACT_URL).as_str(),
                            query,
                        )
                        .expect(zinc_const::panic::DATA_CONVERSION),
                    )
                    .json(&body)
                    .build()
                    .expect(zinc_const::panic::DATA_CONVERSION),
            )
            .await?;

        if !response.status().is_success() {
            anyhow::bail!(Error::ContractUploading(format!(
                "HTTP error ({}) {}",
                response.status(),
                response
                    .text()
                    .await
                    .expect(zinc_const::panic::DATA_CONVERSION),
            )));
        }

        Ok(response
            .json::<zinc_types::PublishResponseBody>()
            .await
            .expect(zinc_const::panic::DATA_CONVERSION))
    }

    ///
    /// Initializes a contract on the Zandbox server.
    ///
    /// Initialization includes making the initial deposit and change-pubkey transaction.
    ///
    pub async fn initialize(
        &self,
        query: zinc_types::InitializeRequestQuery,
        body: zinc_types::InitializeRequestBody,
    ) -> anyhow::Result<zinc_types::InitializeResponseBody> {
        let response = self
            .inner
            .execute(
                self.inner
                    .request(
                        Method::POST,
                        Url::parse_with_params(
                            format!(
                                "{}{}",
                                self.url,
                                zinc_const::zandbox::CONTRACT_INITIALIZE_URL
                            )
                            .as_str(),
                            query,
                        )
                        .expect(zinc_const::panic::DATA_CONVERSION),
                    )
                    .json(&body)
                    .build()
                    .expect(zinc_const::panic::DATA_CONVERSION),
            )
            .await?;

        if !response.status().is_success() {
            anyhow::bail!(Error::ContractUnlocking(format!(
                "HTTP error ({}) {}",
                response.status(),
                response
                    .text()
                    .await
                    .expect(zinc_const::panic::DATA_CONVERSION),
            )));
        }

        Ok(response
            .json::<zinc_types::InitializeResponseBody>()
            .await
            .expect(zinc_const::panic::DATA_CONVERSION))
    }

    ///
    /// Queries a contract on the Zandbox server.
    ///
    pub async fn query(
        &self,
        query: zinc_types::QueryRequestQuery,
        body: zinc_types::QueryRequestBody,
    ) -> anyhow::Result<serde_json::Value> {
        let response = self
            .inner
            .execute(
                self.inner
                    .request(
                        Method::PUT,
                        Url::parse_with_params(
                            format!("{}{}", self.url, zinc_const::zandbox::CONTRACT_QUERY_URL)
                                .as_str(),
                            query,
                        )
                        .expect(zinc_const::panic::DATA_CONVERSION),
                    )
                    .json(&body)
                    .build()
                    .expect(zinc_const::panic::DATA_CONVERSION),
            )
            .await?;

        if !response.status().is_success() {
            anyhow::bail!(Error::ContractQuerying(format!(
                "HTTP error ({}) {}",
                response.status(),
                response
                    .text()
                    .await
                    .expect(zinc_const::panic::DATA_CONVERSION),
            )));
        }

        Ok(response
            .json::<serde_json::Value>()
            .await
            .expect(zinc_const::panic::DATA_CONVERSION))
    }

    ///
    /// Calculates a contract call fee on the Zandbox server.
    ///
    pub async fn fee(
        &self,
        query: zinc_types::FeeRequestQuery,
        body: zinc_types::FeeRequestBody,
    ) -> anyhow::Result<zinc_types::FeeResponseBody> {
        let response = self
            .inner
            .execute(
                self.inner
                    .request(
                        Method::PUT,
                        Url::parse_with_params(
                            format!("{}{}", self.url, zinc_const::zandbox::CONTRACT_FEE_URL)
                                .as_str(),
                            query,
                        )
                        .expect(zinc_const::panic::DATA_CONVERSION),
                    )
                    .json(&body)
                    .build()
                    .expect(zinc_const::panic::DATA_CONVERSION),
            )
            .await?;

        if !response.status().is_success() {
            anyhow::bail!(Error::ContractFeeCalculating(format!(
                "HTTP error ({}) {}",
                response.status(),
                response
                    .text()
                    .await
                    .expect(zinc_const::panic::DATA_CONVERSION),
            )));
        }

        Ok(response
            .json::<zinc_types::FeeResponseBody>()
            .await
            .expect(zinc_const::panic::DATA_CONVERSION))
    }

    ///
    /// Calls a contract on the Zandbox server.
    ///
    pub async fn call(
        &self,
        query: zinc_types::CallRequestQuery,
        body: zinc_types::CallRequestBody,
    ) -> anyhow::Result<serde_json::Value> {
        let response = self
            .inner
            .execute(
                self.inner
                    .request(
                        Method::POST,
                        Url::parse_with_params(
                            format!("{}{}", self.url, zinc_const::zandbox::CONTRACT_CALL_URL)
                                .as_str(),
                            query,
                        )
                        .expect(zinc_const::panic::DATA_CONVERSION),
                    )
                    .json(&body)
                    .build()
                    .expect(zinc_const::panic::DATA_CONVERSION),
            )
            .await?;

        if !response.status().is_success() {
            anyhow::bail!(Error::ContractCalling(format!(
                "HTTP error ({}) {}",
                response.status(),
                response
                    .text()
                    .await
                    .expect(zinc_const::panic::DATA_CONVERSION),
            )));
        }

        Ok(response
            .json::<serde_json::Value>()
            .await
            .expect(zinc_const::panic::DATA_CONVERSION))
    }

    ///
    /// Downloads the contract project source code from the Zandbox server.
    ///
    pub async fn source(
        &self,
        query: zinc_types::SourceRequestQuery,
    ) -> anyhow::Result<zinc_types::SourceResponseBody> {
        let response = self
            .inner
            .execute(
                self.inner
                    .request(
                        Method::GET,
                        Url::parse_with_params(
                            format!("{}{}", self.url, zinc_const::zandbox::PROJECT_SOURCE_URL)
                                .as_str(),
                            query,
                        )
                        .expect(zinc_const::panic::DATA_CONVERSION),
                    )
                    .build()
                    .expect(zinc_const::panic::DATA_CONVERSION),
            )
            .await?;

        if !response.status().is_success() {
            anyhow::bail!(Error::ContractProjectDownloading(format!(
                "HTTP error ({}) {}",
                response.status(),
                response
                    .text()
                    .await
                    .expect(zinc_const::panic::DATA_CONVERSION),
            )));
        }

        Ok(response
            .json::<zinc_types::SourceResponseBody>()
            .await
            .expect(zinc_const::panic::DATA_CONVERSION))
    }
}
