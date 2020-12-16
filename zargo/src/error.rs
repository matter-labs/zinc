//!
//! The Zargo package manager error.
//!

use thiserror::Error;

///
/// The Zargo package manager `publish` subcommand error.
///
#[derive(Debug, Error)]
pub enum Error {
    /// The invalid project name error.
    #[error("project name is missing and cannot be inferred from path {0:?}")]
    ProjectNameInvalid(std::ffi::OsString),

    /// The project name is missing.
    #[error("project name must be specified")]
    ProjectNameMissing,

    /// The invalid project type error.
    #[error("project type must be either `circuit`, `contract`, or `library`, but found `{0}`")]
    ProjectTypeInvalid(String),

    /// The project version is missing.
    #[error("project version must be specified")]
    ProjectVersionMissing,

    /// The project directory does not exist. Use `new` instead.
    #[error("directory {0:?} does not exist")]
    DirectoryDoesNotExist(std::ffi::OsString),

    /// The project directory already exists. Use `init` instead.
    #[error("directory {0:?} already exists")]
    DirectoryAlreadyExists(std::ffi::OsString),

    /// The project has been already initialized.
    #[error("project at path {0:?} is already initialized")]
    ProjectAlreadyInitialized(std::ffi::OsString),

    /// The child process failure exit code.
    #[error("the subprocess failed with status {0}")]
    SubprocessFailure(std::process::ExitStatus),

    /// The child process stdin acquisition has failed.
    #[error("the subprocess stdin acquisition failed")]
    StdinAcquisition,

    /// The invalid network error.
    #[error("invalid network name: {0}")]
    NetworkInvalid(String),

    /// The unimplemented network error.
    #[error("unimplemented network: {0}")]
    NetworkUnimplemented(zksync::Network),

    /// The project is not a contract.
    #[error("not a contract")]
    NotAContract,

    /// The contract method to call is missing.
    #[error("contract method to call must be specified")]
    MethodMissing,

    /// The input file section is missing.
    #[error("input file data must contain section `{0}`")]
    MissingInputSection(String),

    /// The project metadata request failure.
    #[error("project metadata request: {0}")]
    ProjectMetadata(String),

    /// The project uploading request failure.
    #[error("project uploading request: {0}")]
    ProjectUploading(String),

    /// The smart contract uploading request failure.
    #[error("contract uploading request: {0}")]
    ContractUploading(String),

    /// The smart contract unlocking request failure.
    #[error("contract unlocking request: {0}")]
    ContractUnlocking(String),

    /// The smart contract querying request failure.
    #[error("contract querying request: {0}")]
    ContractQuerying(String),

    /// The smart contract fee calculating request failure.
    #[error("contract fee calculating request: {0}")]
    ContractFeeCalculating(String),

    /// The smart contract calling request failure.
    #[error("contract calling request: {0}")]
    ContractCalling(String),

    /// The smart contract project downloading request failure.
    #[error("contract project downloading request: {0}")]
    ContractProjectDownloading(String),

    /// The dependency requires different version of the compiler.
    #[error("project {0}: compiler version mismatch: expected {1}, found {2}")]
    CompilerVersionMismatch(String, String, String),

    /// The command is temporarily unavailable.
    #[error("the proof verification is temporarily unavailable")]
    ProofVerificationUnavailable,
}
