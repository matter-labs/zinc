//!
//! The Zargo package manager `query` subcommand.
//!

use failure::Fail;

use crate::error::file::Error as FileError;

///
/// The Zargo package manager `query` subcommand error.
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
    /// The publish HTTP request error.
    #[fail(display = "HTTP request: {}", _0)]
    HttpRequest(reqwest::Error),
    /// The smart contract server failure.
    #[fail(display = "action failed: {}", _0)]
    ActionFailed(String),
}
