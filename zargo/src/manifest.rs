//!
//! The Zargo manifest.
//!

use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

use failure::Fail;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Manifest {
    pub circuit: Circuit,
}

#[derive(Deserialize)]
pub struct Circuit {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "opening: {}", _0)]
    Opening(io::Error),
    #[fail(display = "reading: {}", _0)]
    Reading(io::Error),
    #[fail(display = "parsing: {}", _0)]
    Parsing(toml::de::Error),
}

impl Manifest {
    pub fn new(path: &PathBuf) -> Result<Self, Error> {
        let mut manifest_str = String::new();
        let mut manifest_file = File::open(path).map_err(Error::Opening)?;
        manifest_file
            .read_to_string(&mut manifest_str)
            .map_err(Error::Reading)?;
        Ok(toml::from_str(&manifest_str).map_err(Error::Parsing)?)
    }
}
