//!
//! The Zinc tester error.
//!

use thiserror::Error;

///
/// The test directory error.
///
#[derive(Debug, Error)]
pub enum Error {
    /// The file type is invalid, that is, the file is neither a data file nor directory.
    #[error("invalid file type: {0:?}")]
    InvalidFileType(std::fs::FileType),
    /// The file extension could not be acquired.
    #[error("file extension getting")]
    GettingFileExtension,
    /// The file extension is invalid.
    #[error("invalid file extension `{0}`")]
    InvalidFileExtension(String),
    /// The method is missing in the test metadata.
    #[error("method missing")]
    MethodMissing,
    /// The method could not be found in the test application.
    #[error("method `{0}` not found")]
    MethodNotFound(String),
    /// The library cannot be run as a standalone application.
    #[error("libraries cannot be run as they have no entry points")]
    CannotRunLibrary,
}
