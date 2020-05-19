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

#[derive(Deserialize)]
pub struct Contract {
    pub name: String,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "creating: {}", _0)]
    Creating(io::Error),
    #[fail(display = "writing: {}", _0)]
    Writing(io::Error),
}

pub static ENTRY_FILE_NAME_DEFAULT: &str = "main";

impl Contract {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }

    pub fn exists_at(path: &PathBuf, name: &str) -> bool {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(super::DIRECTORY_NAME_DEFAULT) {
                path.push(PathBuf::from(super::DIRECTORY_NAME_DEFAULT));
            }
            let file_name = format!("{}.{}", name, super::SOURCE_FILE_EXTENSION_DEFAULT);
            path.push(PathBuf::from(file_name));
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
                ENTRY_FILE_NAME_DEFAULT,
                super::SOURCE_FILE_EXTENSION_DEFAULT
            );
            path.push(PathBuf::from(file_name));
        }

        let mut file = File::create(&path).map_err(Error::Creating)?;
        file.write_all(self.template().as_bytes())
            .map_err(Error::Writing)
    }

    fn template(&self) -> String {
        format!(
            r#"//!
//! The '{}' contract entry.
//!

contract {} {{
    pub fn default(value: u8) -> u8 {{
        dbg!("Zello, World!");

        value
    }}
}}
"#,
            self.name,
            self.name.to_title_case().replace(" ", ""),
        )
    }
}
