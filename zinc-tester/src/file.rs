//!
//! The Zinc tester directory.file.
//!

use std::convert::TryFrom;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

use failure::Fail;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct TestFile {
    pub code: String,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "opening: {}", _0)]
    Opening(io::Error),
    #[fail(display = "metadata: {}", _0)]
    Metadata(io::Error),
    #[fail(display = "reading: {}", _0)]
    Reading(io::Error),
}

impl TryFrom<&PathBuf> for TestFile {
    type Error = Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let mut file = File::open(path).map_err(Error::Opening)?;
        let size = file.metadata().map_err(Error::Metadata)?.len() as usize;
        let mut string = String::with_capacity(size);
        file.read_to_string(&mut string).map_err(Error::Reading)?;
        Ok(Self { code: string })
    }
}
