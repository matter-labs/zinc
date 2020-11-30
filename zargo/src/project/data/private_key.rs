//!
//! The private key file.
//!

use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Context;

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
            path.push(PathBuf::from(Self::file_name()));
        }
        path.exists()
    }

    ///
    /// Writes the contents to a file in the project at the given `path`.
    ///
    pub fn write_to(self, path: &PathBuf) -> anyhow::Result<()> {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(Self::file_name()));
        }

        let mut file = File::create(&path).with_context(|| path.to_string_lossy().to_string())?;
        file.write_all(self.inner.as_bytes())
            .with_context(|| path.to_string_lossy().to_string())?;

        Ok(())
    }

    ///
    /// The private file default template function.
    ///
    fn template() -> String {
        "00".repeat(zinc_const::size::ETH_PRIVATE_KEY)
    }

    ///
    /// Creates a string with the default file name.
    ///
    fn file_name() -> String {
        zinc_const::file_name::PRIVATE_KEY.to_owned()
    }
}

impl TryFrom<&PathBuf> for PrivateKey {
    type Error = anyhow::Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
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

        Ok(Self { inner: buffer })
    }
}
