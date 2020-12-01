//!
//! The source code module error.
//!

use thiserror::Error;

///
/// The source code module error.
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
        "the `{1}.{0}` file cannot be declared at the project root",
        zinc_const::extension::SOURCE,
        zinc_const::file_name::MODULE_ENTRY
    )]
    ModuleEntryInRoot,
    /// The project entry file is deeper than the root directory.
    #[error(
        "the `{1}.{0}` or `{2}.{0}` file must be declared at the project root",
        zinc_const::extension::SOURCE,
        zinc_const::file_name::APPLICATION_ENTRY,
        zinc_const::file_name::LIBRARY_ENTRY
    )]
    ProjectEntryBeyondRoot,
    /// The module entry not found.
    #[error(
        "the `{1}.{0}` file is missing",
        zinc_const::extension::SOURCE,
        zinc_const::file_name::MODULE_ENTRY
    )]
    ModuleEntryNotFound,
    /// The project entry not found. Only for the root directory.
    #[error(
        "the `{1}.{0}` or `{2}.{0}` file is missing",
        zinc_const::extension::SOURCE,
        zinc_const::file_name::APPLICATION_ENTRY,
        zinc_const::file_name::LIBRARY_ENTRY
    )]
    ProjectEntryNotFound,
    /// The source code compiler analysis error, formatted as string.
    #[error("{0}")]
    Compiling(String),
}
