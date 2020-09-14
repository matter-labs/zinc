//!
//! The contract entry file.
//!

use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;

use failure::Fail;
use inflector::Inflector;
use serde_derive::Deserialize;

///
/// The contract source code entry point file representation.
///
#[derive(Deserialize)]
pub struct Contract {
    /// The contract project name.
    pub name: String,
}

///
/// The project contract entry point file error.
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

impl Contract {
    ///
    /// Creates a new file representation instance.
    ///
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }

    ///
    /// Checks if the file exists at the given `path`.
    ///
    pub fn exists_at(path: &PathBuf, name: &str) -> bool {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(zinc_const::directory::SOURCE) {
                path.push(PathBuf::from(zinc_const::directory::SOURCE));
            }
            let file_name = format!("{}.{}", name, zinc_const::extension::SOURCE);
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

        let mut file = File::create(&path).map_err(Error::Creating)?;
        file.write_all(self.template().as_bytes())
            .map_err(Error::Writing)
    }

    ///
    /// The contract main file template function.
    ///
    fn template(&self) -> String {
        format!(
            r#"//!
//! The '{}' contract entry.
//!

contract {} {{
    balance: u248;

    pub fn new(_balance: u248) -> Self {{
        Self {{
            balance: _balance,
        }}
    }}
}}
"#,
            self.name,
            self.name.to_title_case().replace(" ", ""),
        )
    }
}
