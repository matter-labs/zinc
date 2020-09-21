//!
//! The method input arguments file.
//!

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde_derive::Deserialize;
use serde_json::Map as JsonMap;
use serde_json::Value as JsonValue;

use crate::file::error::Error;

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
    pub fn try_from_path(path: &PathBuf, method: &str) -> Result<Self, Error<serde_json::Error>> {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(Self::file_name(method)));
        }

        let mut file =
            File::open(path).map_err(|error| Error::Opening(Self::file_name(method), error))?;
        let size = file
            .metadata()
            .map_err(|error| Error::Metadata(Self::file_name(method), error))?
            .len() as usize;

        let mut buffer = String::with_capacity(size);
        file.read_to_string(&mut buffer)
            .map_err(|error| Error::Reading(Self::file_name(method), error))?;

        let inner = serde_json::from_str(buffer.as_str())
            .map_err(|error| Error::Parsing(Self::file_name(method), error))?;

        Ok(Self { inner })
    }

    ///
    /// Gets the transaction argument from the JSON.
    ///
    /// Should only be called for mutable methods (`call` command) where the transaction is mandatory.
    ///
    pub fn get_tx(&self) -> Option<JsonMap<String, JsonValue>> {
        match self.inner {
            JsonValue::Object(ref map) => match map.get("tx").cloned() {
                Some(JsonValue::Object(map)) => Some(map),
                _ => None,
            },
            _ => None,
        }
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
