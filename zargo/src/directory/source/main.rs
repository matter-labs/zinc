//!
//! The circuit `main.zn` directory.file.
//!

//!
//! The Zargo manifest.
//!

use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;

use failure::Fail;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Main {
    pub circuit_name: String,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "creating: {}", _0)]
    Creating(io::Error),
    #[fail(display = "writing: {}", _0)]
    Writing(io::Error),
}

pub static FILE_NAME_DEFAULT: &str = "main.zn";

impl Main {
    pub fn new(circuit_name: &str) -> Self {
        Self {
            circuit_name: circuit_name.to_owned(),
        }
    }

    pub fn exists_at(path: &PathBuf) -> bool {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(super::DIRECTORY_NAME_DEFAULT) {
                path.push(PathBuf::from(super::DIRECTORY_NAME_DEFAULT));
            }
            path.push(PathBuf::from(FILE_NAME_DEFAULT));
        }
        path.exists()
    }

    pub fn write_to(self, path: &PathBuf) -> Result<(), Error> {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(super::DIRECTORY_NAME_DEFAULT) {
                path.push(PathBuf::from(super::DIRECTORY_NAME_DEFAULT));
            }
            path.push(PathBuf::from(FILE_NAME_DEFAULT));
        }

        let mut file = File::create(&path).map_err(Error::Creating)?;
        file.write_all(self.template().as_bytes())
            .map_err(Error::Writing)
    }

    fn template(&self) -> String {
        format!(
            r#"//!
//! The '{}' main module.
//!

fn main(witness: u8) -> u8 {{
    dbg!("Zello, World!");

    witness
}}
"#,
            self.circuit_name
        )
    }
}
