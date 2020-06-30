//!
//! The circuit `main.zn` file.
//!

use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;

use failure::Fail;
use serde_derive::Deserialize;

///
/// The circuit source code entry point file representation.
///
#[derive(Deserialize)]
pub struct Circuit {
    /// The circuit project name.
    pub name: String,
}

///
/// The project circuit entry point file error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The file creating error.
    #[fail(display = "creating: {}", _0)]
    Creating(io::Error),
    /// The file writing error.
    #[fail(display = "writing: {}", _0)]
    Writing(io::Error),
}

impl Circuit {
    ///
    /// Creates a new file representation instance.
    ///
    pub fn new(circuit_name: &str) -> Self {
        Self {
            name: circuit_name.to_owned(),
        }
    }

    ///
    /// Checks if the file exists at the given `path`.
    ///
    pub fn exists_at(path: &PathBuf) -> bool {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(zinc_const::zargo::SOURCE_DIRECTORY_SUBPATH) {
                path.push(PathBuf::from(zinc_const::zargo::SOURCE_DIRECTORY_SUBPATH));
            }
            let file_name = format!(
                "{}.{}",
                zinc_const::file_names::APPLICATION_ENTRY,
                zinc_const::extensions::SOURCE
            );
            path.push(PathBuf::from(file_name));
        }
        path.exists()
    }

    ///
    /// Creates the file at the given `path`.
    ///
    pub fn write_to(self, path: &PathBuf) -> Result<(), Error> {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(zinc_const::zargo::SOURCE_DIRECTORY_SUBPATH) {
                path.push(PathBuf::from(zinc_const::zargo::SOURCE_DIRECTORY_SUBPATH));
            }
            let file_name = format!(
                "{}.{}",
                zinc_const::file_names::APPLICATION_ENTRY,
                zinc_const::extensions::SOURCE,
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
