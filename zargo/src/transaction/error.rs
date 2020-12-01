//!
//! The transaction error.
//!

use thiserror::Error;

///
/// The transaction error.
///
#[derive(Debug, Error)]
pub enum Error {
    /// A required transaction field is missing.
    #[error("parsing: {}", _0)]
    Parsing(zinc_types::TransactionError),
    /// The transaction token is invalid.
    #[error("token is invalid and cannot be resolved")]
    TokenNotFound,
    /// The transaction fee getting error.
    #[error("transaction fee getting error: {}", _0)]
    FeeGetting(zksync::error::ClientError),
    /// The account info retrieving error.
    #[error("account info retrieving error: {}", _0)]
    AccountInfoRetrieving(zksync::error::ClientError),
    /// The transaction signing error.
    #[error("signing error: {}", _0)]
    TransactionSigning(zksync_eth_signer::error::SignerError),
}

impl From<zinc_types::TransactionError> for Error {
    fn from(inner: zinc_types::TransactionError) -> Self {
        Self::Parsing(inner)
    }
}
