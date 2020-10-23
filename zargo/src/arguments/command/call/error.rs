//!
//! The Zargo package manager `call` subcommand.
//!

use failure::Fail;

use crate::error::file::Error as FileError;
use crate::transaction::error::Error as TransactionError;

///
/// The Zargo package manager `call` subcommand error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The ETH address is invalid.
    #[fail(display = "invalid ETH address: {}", _0)]
    InvalidContractAddress(rustc_hex::FromHexError),
    /// The invalid network error.
    #[fail(display = "invalid network name: {}", _0)]
    NetworkInvalid(String),
    /// The unimplemented network error.
    #[fail(display = "unimplemented network: {}", _0)]
    NetworkUnimplemented(zksync::Network),
    /// The manifest file error.
    #[fail(display = "manifest {}", _0)]
    Manifest(zinc_manifest::Error),
    /// The project is not a contract.
    #[fail(display = "not a contract")]
    NotAContract,
    /// The input file error.
    #[fail(display = "input file {}", _0)]
    InputFile(FileError<serde_json::Error>),
    /// The input file data is invalid.
    #[fail(display = "invalid input file data")]
    InvalidInputData,
    /// The private key file error.
    #[fail(display = "private key file {}", _0)]
    PrivateKeyFile(FileError),
    /// The sender private key is invalid.
    #[fail(display = "sender private key is invalid: {}", _0)]
    SenderPrivateKeyInvalid(rustc_hex::FromHexError),
    /// The sender address cannot be derived from the private key.
    #[fail(
        display = "could not derive the ETH address from the private key: {}",
        _0
    )]
    SenderAddressDeriving(anyhow::Error),
    /// The wallet initialization error.
    #[fail(display = "wallet initialization: {}", _0)]
    WalletInitialization(zksync::error::ClientError),
    /// The transaction signing error.
    #[fail(display = "transaction: {}", _0)]
    Transaction(TransactionError),
    /// The publish HTTP request error.
    #[fail(display = "HTTP request: {}", _0)]
    HttpRequest(reqwest::Error),
    /// The smart contract server failure.
    #[fail(display = "action failed: {}", _0)]
    ActionFailed(String),
}
