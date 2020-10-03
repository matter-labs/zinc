//!
//! The contract resource POST `initialize` error.
//!

use std::fmt;

use actix_web::http::StatusCode;
use actix_web::ResponseError;

use zksync_types::TokenId;

///
/// The contract resource POST `initialize` error.
///
#[derive(Debug)]
pub enum Error {
    /// The contract with the specified address is not found in the server cache.
    ContractNotFound(String),
    /// Token ID cannot be resolved by zkSync.
    TokenNotFound(TokenId),
    /// Failed to execute the initial transfer transaction.
    InitialTransfer(String),
    /// Could not get the account ID.
    AccountId,
    /// Failed to execute the change-pubkey transaction.
    ChangePubkey(String),

    /// The PostgreSQL database error.
    Database(sqlx::Error),
    /// The ZkSync server client error.
    ZkSyncClient(zksync::error::ClientError),
}

impl From<sqlx::Error> for Error {
    fn from(inner: sqlx::Error) -> Self {
        Self::Database(inner)
    }
}

impl From<zksync::error::ClientError> for Error {
    fn from(inner: zksync::error::ClientError) -> Self {
        Self::ZkSyncClient(inner)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::ContractNotFound(..) => StatusCode::NOT_FOUND,
            Self::TokenNotFound(..) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::InitialTransfer(..) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::AccountId => StatusCode::UNPROCESSABLE_ENTITY,
            Self::ChangePubkey(..) => StatusCode::UNPROCESSABLE_ENTITY,

            Self::Database(..) => StatusCode::SERVICE_UNAVAILABLE,
            Self::ZkSyncClient(..) => StatusCode::SERVICE_UNAVAILABLE,
        }
    }
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            Self::ContractNotFound(address) => {
                format!("Contract with address {} not found", address)
            }
            Self::TokenNotFound(token_id) => format!("Token ID {} cannot be resolved", token_id),
            Self::InitialTransfer(inner) => format!("Initial transfer: {}", inner),
            Self::AccountId => "Could not get the contract account ID".to_owned(),
            Self::ChangePubkey(inner) => format!("Changing the contract public key: {}", inner),

            Self::Database(inner) => format!("Database: {:?}", inner),
            Self::ZkSyncClient(inner) => format!("ZkSync: {:?}", inner),
        };

        log::warn!("{}", error);
        write!(f, "{}", error)
    }
}
