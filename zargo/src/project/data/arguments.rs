//!
//! The method input arguments file.
//!

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde_derive::Deserialize;
use serde_json::Value as JsonValue;

use crate::error::file::Error as FileError;

///
/// The method input arguments file representation.
///
#[derive(Deserialize)]
pub struct Arguments {
    /// The arguments JSON.
    pub inner: JsonValue,
}

impl Arguments {
    ///
    /// Parses the arguments file at `path`.
    ///
    pub fn try_from_path(
        path: &PathBuf,
        method: &str,
    ) -> Result<Self, FileError<serde_json::Error>> {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(Self::file_name(method)));
        }

        let mut file =
            File::open(path).map_err(|error| FileError::Opening(Self::file_name(method), error))?;
        let size = file
            .metadata()
            .map_err(|error| FileError::Metadata(Self::file_name(method), error))?
            .len() as usize;

        let mut buffer = String::with_capacity(size);
        file.read_to_string(&mut buffer)
            .map_err(|error| FileError::Reading(Self::file_name(method), error))?;

        let inner = serde_json::from_str(buffer.as_str())
            .map_err(|error| FileError::Parsing(Self::file_name(method), error))?;

        Ok(Self { inner })
    }

    ///
    /// Creates a string with the default file name.
    ///
    fn file_name(method: &str) -> String {
        format!(
            "{}_{}.{}",
            zinc_const::file_name::WITNESS,
            method,
            zinc_const::extension::JSON,
        )
    }
}
