//!
//! The contract entry file.
//!

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Context;
use inflector::Inflector;
use serde::Deserialize;

///
/// The contract source code entry point file representation.
///
#[derive(Deserialize)]
pub struct Contract {
    /// The contract project name.
    pub name: String,
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
    /// Checks if the file exists in the project at the given `path`.
    ///
    pub fn exists_at(path: &PathBuf) -> bool {
        Self::append_default(path).exists()
    }

    ///
    /// Creates the file in the project at the given `path`.
    ///
    pub fn write_to(self, path: &PathBuf) -> anyhow::Result<()> {
        let path = Self::append_default(path);
        let mut file = File::create(&path).with_context(|| path.to_string_lossy().to_string())?;
        file.write_all(self.template().as_bytes())
            .with_context(|| path.to_string_lossy().to_string())?;

        Ok(())
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
            path.push(PathBuf::from(Self::file_name()));
        }
        path
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
    pub value: u64;

    pub fn new(value: u64) -> Self {{
        Self {{
            value: value,
        }}
    }}
}}
"#,
            self.name,
            self.name.to_title_case().replace(" ", ""),
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
