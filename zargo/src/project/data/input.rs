//!
//! The application input file representation.
//!

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use anyhow::Context;
use serde::Deserialize;

///
/// The application input file representation.
///
#[derive(Deserialize)]
pub struct Input {
    /// The arguments JSON.
    pub inner: serde_json::Value,
}

impl Input {
    ///
    /// Parses the arguments file at `path`.
    ///
    pub fn try_from_path(path: &PathBuf) -> anyhow::Result<Self> {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(Self::file_name()));
        }

        let mut file = File::open(&path).with_context(|| path.to_string_lossy().to_string())?;
        let size = file
            .metadata()
            .with_context(|| path.to_string_lossy().to_string())?
            .len() as usize;

        let mut buffer = String::with_capacity(size);
        file.read_to_string(&mut buffer)
            .with_context(|| path.to_string_lossy().to_string())?;

        let inner = serde_json::from_str(buffer.as_str())
            .with_context(|| path.to_string_lossy().to_string())?;

        Ok(Self { inner })
    }

    ///
    /// Creates a string with the default file name.
    ///
    fn file_name() -> String {
        format!(
            "{}.{}",
            zinc_const::file_name::INPUT,
            zinc_const::extension::JSON,
        )
    }
}
