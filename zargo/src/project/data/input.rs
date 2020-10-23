//!
//! The application input file representation.
//!

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde::Deserialize;
use serde_json::Value as JsonValue;

use crate::error::file::Error as FileError;

///
/// The application input file representation.
///
#[derive(Deserialize)]
pub struct Input {
    /// The arguments JSON.
    pub inner: JsonValue,
}

impl Input {
    ///
    /// Parses the arguments file at `path`.
    ///
    pub fn try_from_path(path: &PathBuf) -> Result<Self, FileError<serde_json::Error>> {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(Self::file_name()));
        }

        let mut file =
            File::open(path).map_err(|error| FileError::Opening(Self::file_name(), error))?;
        let size = file
            .metadata()
            .map_err(|error| FileError::Metadata(Self::file_name(), error))?
            .len() as usize;

        let mut buffer = String::with_capacity(size);
        file.read_to_string(&mut buffer)
            .map_err(|error| FileError::Reading(Self::file_name(), error))?;

        let inner = serde_json::from_str(buffer.as_str())
            .map_err(|error| FileError::Parsing(Self::file_name(), error))?;

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
