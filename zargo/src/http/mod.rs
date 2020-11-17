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
    /// Publishes a contract to the Zandbox server.
    ///
    pub async fn publish(
        &self,
        query: zinc_zksync::PublishRequestQuery,
        body: zinc_zksync::PublishRequestBody,
    ) -> anyhow::Result<zinc_zksync::PublishResponseBody> {
        let response = self
            .inner
            .execute(
                self.inner
                    .request(
                        Method::POST,
                        Url::parse_with_params(
                            format!("{}{}", self.url, zinc_const::zandbox::CONTRACT_PUBLISH_URL)
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
            .json::<zinc_zksync::PublishResponseBody>()
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
        query: zinc_zksync::InitializeRequestQuery,
        body: zinc_zksync::InitializeRequestBody,
    ) -> anyhow::Result<zinc_zksync::InitializeResponseBody> {
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
            .json::<zinc_zksync::InitializeResponseBody>()
            .await
            .expect(zinc_const::panic::DATA_CONVERSION))
    }

    ///
    /// Queries a contract on the Zandbox server.
    ///
    pub async fn query(
        &self,
        query: zinc_zksync::QueryRequestQuery,
        body: zinc_zksync::QueryRequestBody,
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
        query: zinc_zksync::FeeRequestQuery,
        body: zinc_zksync::FeeRequestBody,
    ) -> anyhow::Result<zinc_zksync::FeeResponseBody> {
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
            .json::<zinc_zksync::FeeResponseBody>()
            .await
            .expect(zinc_const::panic::DATA_CONVERSION))
    }

    ///
    /// Calls a contract on the Zandbox server.
    ///
    pub async fn call(
        &self,
        query: zinc_zksync::CallRequestQuery,
        body: zinc_zksync::CallRequestBody,
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
        query: zinc_zksync::SourceRequestQuery,
    ) -> anyhow::Result<zinc_zksync::SourceResponseBody> {
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
            .json::<zinc_zksync::SourceResponseBody>()
            .await
            .expect(zinc_const::panic::DATA_CONVERSION))
    }
}
