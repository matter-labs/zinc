//!
//! The transaction error.
//!

use failure::Fail;

///
/// The transaction error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// A required transaction field is missing.
    #[fail(display = "parsing: {}", _0)]
    Parsing(zinc_zksync::TransactionMsgError),
    /// The transaction token is invalid.
    #[fail(display = "token is invalid and cannot be resolved")]
    TokenNotFound,
    /// The transaction fee getting error.
    #[fail(display = "transaction fee getting error: {}", _0)]
    FeeGetting(zksync::error::ClientError),
    /// The account info retrieving error.
    #[fail(display = "account info retrieving error: {}", _0)]
    AccountInfoRetrieving(zksync::error::ClientError),
    /// The transaction signing error.
    #[fail(display = "signing error: {}", _0)]
    TransactionSigning(zksync_eth_signer::error::SignerError),
}

impl From<zinc_zksync::TransactionMsgError> for Error {
    fn from(inner: zinc_zksync::TransactionMsgError) -> Self {
        Self::Parsing(inner)
    }
}
