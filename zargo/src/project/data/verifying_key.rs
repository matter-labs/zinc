//!
//! The verifying key file.
//!

use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use anyhow::Context;

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
    type Error = anyhow::Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(zinc_const::directory::DATA) {
                path.push(PathBuf::from(zinc_const::directory::DATA));
            }
            path.push(PathBuf::from(Self::file_name()));
        }

        let mut file = File::open(&path).with_context(|| path.to_string_lossy().to_string())?;
        let size = file
            .metadata()
            .with_context(|| path.to_string_lossy().to_string())?
            .len() as usize;

        let mut buffer = Vec::with_capacity(size);
        file.read_to_end(&mut buffer)
            .with_context(|| path.to_string_lossy().to_string())?;

        Ok(Self { inner: buffer })
    }
}
