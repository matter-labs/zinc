//!
//! The verifying key file.
//!

use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::error::file::Error as FileError;

///
/// The verifying key file representation.
///
pub struct VerifyingKey {
    /// The file contents.
    pub inner: Vec<u8>,
}

impl VerifyingKey {
    ///
    /// Creates a string with the default file name.
    ///
    fn file_name() -> String {
        zinc_const::file_name::VERIFYING_KEY.to_owned()
    }
}

impl TryFrom<&PathBuf> for VerifyingKey {
    type Error = FileError;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(zinc_const::directory::DATA) {
                path.push(PathBuf::from(zinc_const::directory::DATA));
            }
            path.push(PathBuf::from(Self::file_name()));
        }

        let mut file =
            File::open(path).map_err(|error| FileError::Opening(Self::file_name(), error))?;
        let size = file
            .metadata()
            .map_err(|error| FileError::Metadata(Self::file_name(), error))?
            .len() as usize;

        let mut buffer = Vec::with_capacity(size);
        file.read_to_end(&mut buffer)
            .map_err(|error| FileError::Reading(Self::file_name(), error))?;

        Ok(Self { inner: buffer })
    }
}
