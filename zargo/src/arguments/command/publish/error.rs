//!
//! The Zargo package manager `publish` subcommand.
//!

use failure::Fail;

use zinc_zksync::SourceError;

use crate::error::directory::Error as DirectoryError;
use crate::error::file::Error as FileError;
use crate::executable::compiler::Error as CompilerError;
use crate::executable::virtual_machine::Error as VirtualMachineError;
use crate::transaction::error::Error as TransactionError;

///
/// The Zargo package manager `publish` subcommand error.
///
#[derive(Debug, Fail)]
pub enum Error {
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
    /// The source code error.
    #[fail(display = "source code {}", _0)]
    Source(SourceError),
    /// The project binary build directory error.
    #[fail(display = "build directory {}", _0)]
    BuildDirectory(DirectoryError),
    /// The project template, keys, and other auxiliary data directory error.
    #[fail(display = "data directory {}", _0)]
    DataDirectory(DirectoryError),
    /// The compiler process error.
    #[fail(display = "compiler {}", _0)]
    Compiler(CompilerError),
    /// The virtual machine process error.
    #[fail(display = "virtual machine {}", _0)]
    VirtualMachine(VirtualMachineError),
    /// The contract bytecode binary file error.
    #[fail(display = "bytecode binary file {}", _0)]
    BinaryFile(FileError),
    /// The input file error.
    #[fail(display = "input file {}", _0)]
    InputFile(FileError<serde_json::Error>),
    /// The input file data is invalid.
    #[fail(display = "invalid input file data")]
    InvalidInputData,
    /// The constructor arguments not found.
    #[fail(display = "constructor arguments not found")]
    ConstructorArgumentsNotFound,
    /// The verifying key file error.
    #[fail(display = "verifying key file {}", _0)]
    VerifyingKeyFile(FileError),
    /// The publish HTTP request error.
    #[fail(display = "HTTP request: {}", _0)]
    HttpRequest(reqwest::Error),
    /// The smart contract server failure.
    #[fail(display = "action failed: {}", _0)]
    ActionFailed(String),
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
    /// The initial deposit amount is invalid.
    #[fail(display = "initial deposit amount: {}", _0)]
    InitialDepositAmount(zinc_math::BigIntError),
    /// The wallet initialization error.
    #[fail(display = "wallet initialization: {}", _0)]
    WalletInitialization(zksync::error::ClientError),
    /// The transaction signing error.
    #[fail(display = "transaction: {}", _0)]
    Transaction(TransactionError),
}
