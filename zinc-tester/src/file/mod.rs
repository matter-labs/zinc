//!
//! The Zinc tester file.
//!

pub mod error;

use std::convert::TryFrom;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

use serde::Deserialize;

use self::error::Error;

///
/// The integration test file.
///
#[derive(Debug, Deserialize, PartialEq)]
pub struct File {
    /// The test file source code.
    pub code: String,
}

impl TryFrom<&PathBuf> for File {
    type Error = Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let mut file = fs::File::open(path).map_err(Error::Opening)?;

        let size = file.metadata().map_err(Error::Metadata)?.len() as usize;
        let mut string = String::with_capacity(size);
        file.read_to_string(&mut string).map_err(Error::Reading)?;

        Ok(Self { code: string })
    }
}
