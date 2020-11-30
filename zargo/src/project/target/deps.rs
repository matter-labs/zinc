//!
//! The project `target/deps` directory.
//!

use std::fs;
use std::path::PathBuf;

use anyhow::Context;

///
/// The project `target/deps` directory.
///
pub struct Directory {}

impl Directory {
    ///
    /// If the `path` does not end with the directory subpath, appends the subpath to the `path`.
    ///
    pub fn path(path: &PathBuf) -> PathBuf {
        let mut path = path.to_owned();
        if path.is_dir() && !path.ends_with(zinc_const::directory::TARGET_DEPS) {
            path.push(PathBuf::from(zinc_const::directory::TARGET_DEPS));
        }
        path
    }

    ///
    /// Creates a directory with all its parent directories if it does not exist.
    ///
    pub fn create(path: &PathBuf) -> anyhow::Result<()> {
        Ok(fs::create_dir_all(&Self::path(path))
            .with_context(|| path.to_string_lossy().to_string())?)
    }
}
