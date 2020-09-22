//!
//! The private key file.
//!

use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use crate::error::file::Error as FileError;

///
/// The private key file representation.
///
pub struct PrivateKey {
    /// The file contents.
    pub inner: String,
}

impl Default for PrivateKey {
    fn default() -> Self {
        Self {
            inner: Self::template(),
        }
    }
}

impl PrivateKey {
    ///
    /// Checks if the file exists in the project at the given `path`.
    ///
    pub fn exists_at(path: &PathBuf) -> bool {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(zinc_const::directory::DATA) {
                path.push(PathBuf::from(zinc_const::directory::DATA));
            }
            path.push(PathBuf::from(Self::file_name()));
        }
        path.exists()
    }

    ///
    /// Writes the contents to a file in the project at the given `path`.
    ///
    pub fn write_to(self, path: &PathBuf) -> Result<(), FileError> {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(zinc_const::directory::DATA) {
                path.push(PathBuf::from(zinc_const::directory::DATA));
            }
            path.push(PathBuf::from(Self::file_name()));
        }

        let mut file =
            File::create(&path).map_err(|error| FileError::Creating(Self::file_name(), error))?;
        file.write_all(self.inner.as_bytes())
            .map_err(|error| FileError::Writing(Self::file_name(), error))
    }

    ///
    /// The private file default template function.
    ///
    fn template() -> String {
        "0".repeat(zinc_const::size::ETH_PRIVATE_KEY * 2)
    }

    ///
    /// Creates a string with the default file name.
    ///
    fn file_name() -> String {
        zinc_const::file_name::PRIVATE_KEY.to_owned()
    }
}

impl TryFrom<&PathBuf> for PrivateKey {
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

        let mut buffer = String::with_capacity(size);
        file.read_to_string(&mut buffer)
            .map_err(|error| FileError::Reading(Self::file_name(), error))?;

        Ok(Self { inner: buffer })
    }
}
