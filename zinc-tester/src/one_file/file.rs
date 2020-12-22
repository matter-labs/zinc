//!
//! The Zinc tester file.
//!

use std::convert::TryFrom;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

use anyhow::Context;

use crate::error::Error;

///
/// The integration test file.
///
#[derive(Debug)]
pub struct File {
    /// The test file source code.
    pub code: String,
}

impl TryFrom<&PathBuf> for File {
    type Error = anyhow::Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let mut file = fs::File::open(path).with_context(|| path.to_string_lossy().to_string())?;

        let size = file
            .metadata()
            .with_context(|| path.to_string_lossy().to_string())?
            .len() as usize;
        let file_extension = path
            .extension()
            .ok_or(Error::GettingFileExtension)
            .with_context(|| path.to_string_lossy().to_string())?;
        if file_extension != zinc_const::extension::SOURCE {
            return Err(Error::InvalidFileExtension(
                file_extension.to_string_lossy().to_string(),
            ))
            .with_context(|| path.to_string_lossy().to_string());
        }

        let mut string = String::with_capacity(size);
        file.read_to_string(&mut string)
            .with_context(|| path.to_string_lossy().to_string())?;

        Ok(Self { code: string })
    }
}
