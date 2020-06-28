//!
//! The circuit `main.zn` file.
//!

use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;

use failure::Fail;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Circuit {
    pub name: String,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "creating: {}", _0)]
    Creating(io::Error),
    #[fail(display = "writing: {}", _0)]
    Writing(io::Error),
}

impl Circuit {
    pub fn new(circuit_name: &str) -> Self {
        Self {
            name: circuit_name.to_owned(),
        }
    }

    pub fn exists_at(path: &PathBuf) -> bool {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(super::DIRECTORY_NAME_DEFAULT) {
                path.push(PathBuf::from(super::DIRECTORY_NAME_DEFAULT));
            }
            path.push(PathBuf::from(
                zinc_const::source::APPLICATION_ENTRY_FILE_NAME,
            ));
        }
        path.exists()
    }

    pub fn write_to(self, path: &PathBuf) -> Result<(), Error> {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(super::DIRECTORY_NAME_DEFAULT) {
                path.push(PathBuf::from(super::DIRECTORY_NAME_DEFAULT));
            }
            let file_name = format!(
                "{}.{}",
                zinc_const::source::APPLICATION_ENTRY_FILE_NAME,
                zinc_const::source::FILE_EXTENSION
            );
            path.push(PathBuf::from(file_name));
        }

        let mut file = File::create(&path).map_err(Error::Creating)?;
        file.write_all(self.template().as_bytes())
            .map_err(Error::Writing)
    }

    ///
    /// The circuit main file template function.
    ///
    fn template(&self) -> String {
        format!(
            r#"//!
//! The '{}' circuit entry.
//!

fn main(witness: u8) -> u8 {{
    dbg!("Zello, World!");

    witness
}}
"#,
            self.name
        )
    }
}
