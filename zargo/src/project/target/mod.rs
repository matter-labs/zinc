//!
//! The project `target` directory.
//!

pub mod bytecode;
pub mod deps;

use std::fs;
use std::path::PathBuf;

use anyhow::Context;

///
/// The project `target` directory.
///
pub struct Directory {}

impl Directory {
    ///
    /// If the `path` does not end with the directory subpath, appends the subpath to the `path`.
    ///
    pub fn path(path: &PathBuf, is_release: bool) -> PathBuf {
        let target = if is_release {
            zinc_const::directory::TARGET_RELEASE
        } else {
            zinc_const::directory::TARGET_DEBUG
        };

        let mut path = path.to_owned();
        if path.is_dir() && !path.ends_with(target) {
            path.push(PathBuf::from(target));
        }
        path
    }

    ///
    /// Creates a directory with all its parent directories if it does not exist.
    ///
    pub fn create(path: &PathBuf, is_release: bool) -> anyhow::Result<()> {
        Ok(fs::create_dir_all(&Self::path(path, is_release))
            .with_context(|| path.to_string_lossy().to_string())?)
    }

    ///
    /// Removes the directory with all its child directories.
    ///
    pub fn remove(path: &PathBuf) -> anyhow::Result<()> {
        let mut path = path.to_owned();
        if path.is_dir() && !path.ends_with(zinc_const::directory::TARGET) {
            path.push(PathBuf::from(zinc_const::directory::TARGET));
        }

        if path.exists() {
            fs::remove_dir_all(&path).with_context(|| path.to_string_lossy().to_string())?;
        }

        Ok(())
    }
}
