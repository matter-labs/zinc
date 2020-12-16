//!
//! The source code module error.
//!

use thiserror::Error;

///
/// The source code module error kind.
///
#[derive(Debug, Error)]
pub enum Error {
    /// Failed to get the file type, that is, file or directory.
    #[error("file type is unknown")]
    FileTypeUnknown,
    /// The file has no extension.
    #[error("file extension not found")]
    ExtensionNotFound,
    /// The file extension is not the one we are looking for.
    #[error("file extension {0:?} is invalid")]
    ExtensionInvalid(std::ffi::OsString),
    /// The file has no stem, that is, name without the extension.
    #[error("file or directory stem not found")]
    StemNotFound,
    /// The module entry is in the root directory. Only the application entry allowed there.
    #[error(
        "the `{}.{}` file cannot be declared at the project root",
        zinc_const::file_name::MODULE_ENTRY,
        zinc_const::extension::SOURCE
    )]
    ModuleEntryInRoot,
    /// The application entry file is deeper than the root directory.
    #[error(
        "the `{}.{}` file must be declared at the project root",
        zinc_const::file_name::APPLICATION_ENTRY,
        zinc_const::extension::SOURCE
    )]
    ApplicationEntryBeyondRoot,
    /// The module entry not found.
    #[error(
        "the `{}.{}` file is missing",
        zinc_const::file_name::MODULE_ENTRY,
        zinc_const::extension::SOURCE
    )]
    ModuleEntryNotFound,
    /// The application entry not found. Only for the root directory.
    #[error(
        "the `{}.{}` file is missing",
        zinc_const::file_name::APPLICATION_ENTRY,
        zinc_const::extension::SOURCE
    )]
    ApplicationEntryNotFound,
}
