//!
//! The source code module error.
//!

use thiserror::Error;

// TODO: add path contexts

///
/// The source code module error.
///
#[derive(Debug, Error)]
pub enum Error {
    /// The file system I/O error.
    #[error("file system: {0}")]
    FileSystem(#[from] std::io::Error),
    /// The `*.toml` file parsing error.
    #[error("parsing: {0}")]
    Parsing(#[from] toml::de::Error),
}
