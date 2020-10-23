//!
//! The circuit `main.zn` file.
//!

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use serde::Deserialize;

use crate::error::file::Error as FileError;

///
/// The circuit source code entry point file representation.
///
#[derive(Deserialize)]
pub struct Circuit {
    /// The circuit project name.
    pub name: String,
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
    /// Checks if the file exists in the project at the given `path`.
    ///
    pub fn exists_at(path: &PathBuf) -> bool {
        Self::append_default(path).exists()
    }

    ///
    /// Creates the file in the project at the given `path`.
    ///
    pub fn write_to(self, path: &PathBuf) -> Result<(), FileError> {
        let path = Self::append_default(path);
        let mut file =
            File::create(&path).map_err(|error| FileError::Creating(Self::file_name(), error))?;
        file.write_all(self.template().as_bytes())
            .map_err(|error| FileError::Writing(Self::file_name(), error))
    }

    ///
    /// If the path is a directory, appends the missing elements by default.
    ///
    fn append_default(path: &PathBuf) -> PathBuf {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(zinc_const::directory::SOURCE) {
                path.push(PathBuf::from(zinc_const::directory::SOURCE));
            }
            let file_name = format!(
                "{}.{}",
                zinc_const::file_name::APPLICATION_ENTRY,
                zinc_const::extension::SOURCE,
            );
            path.push(PathBuf::from(file_name));
        }
        path
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

    ///
    /// Creates a string with the default file name.
    ///
    fn file_name() -> String {
        format!(
            "{}.{}",
            zinc_const::file_name::APPLICATION_ENTRY,
            zinc_const::extension::SOURCE,
        )
    }
}
