//!
//! The Zargo manifest.
//!

use std::convert::TryFrom;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
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
    #[fail(display = "metadata: {}", _0)]
    Metadata(io::Error),
    #[fail(display = "reading: {}", _0)]
    Reading(io::Error),
    #[fail(display = "parsing: {}", _0)]
    Parsing(toml::de::Error),
    #[fail(display = "creating: {}", _0)]
    Creating(io::Error),
    #[fail(display = "writing: {}", _0)]
    Writing(io::Error),
}

pub static FILE_NAME_DEFAULT: &str = "Zargo.toml";

impl Manifest {
    pub fn new(circuit_name: &str) -> Self {
        Self {
            circuit: Circuit {
                name: circuit_name.to_owned(),
                version: "0.1.0".to_owned(),
            },
        }
    }

    pub fn exists_at(path: &PathBuf) -> bool {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(FILE_NAME_DEFAULT));
        }
        path.exists()
    }

    pub fn write_to(self, path: &PathBuf) -> Result<(), Error> {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(FILE_NAME_DEFAULT));
        }
        let mut zargo_file = File::create(&path).map_err(Error::Creating)?;
        zargo_file
            .write_all(self.template().as_bytes())
            .map_err(Error::Writing)
    }

    fn template(&self) -> String {
        format!(
            r#"[circuit]
name = "{}"
version = "0.1.0"
"#,
            self.circuit.name
        )
    }
}

impl TryFrom<&PathBuf> for Manifest {
    type Error = Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(FILE_NAME_DEFAULT));
        }

        let mut file = File::open(path).map_err(Error::Opening)?;
        let size = file.metadata().map_err(Error::Metadata)?.len() as usize;

        let mut buffer = String::with_capacity(size);
        file.read_to_string(&mut buffer).map_err(Error::Reading)?;

        Ok(toml::from_str(&buffer).map_err(Error::Parsing)?)
    }
}
