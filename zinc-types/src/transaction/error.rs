//!
//! The transaction error.
//!

use thiserror::Error;

use zksync_types::TokenId;

///
/// The transaction error.
///
#[derive(Debug, Error)]
pub enum Error {
    /// The transaction argument is invalid.
    #[error("expected a JSON object, found {0}")]
    ArgumentInvalidFormat(serde_json::Value),
    /// The BigInt string field parsing error.
    #[error("parsing {0}: {1}")]
    FieldParsingLongInteger(&'static str, zinc_math::Error),
    /// The hexadecimal string field parsing error.
    #[error("parsing {0}: {1}")]
    FieldParsingHex(&'static str, rustc_hex::FromHexError),
    /// A transaction field is missing.
    #[error("field is missing")]
    FieldMissing(&'static str),
    /// A transaction field is not a string.
    #[error("field must be a string")]
    FieldNotAString(&'static str),
    /// Unsupported transaction type..
    #[error("the transaction type `{0}` is not supported")]
    UnsupportedTransaction(&'static str),
    /// The token is unknown or unsupported.
    #[error("the token ID {0} is not supported")]
    UnsupportedToken(TokenId),
}
