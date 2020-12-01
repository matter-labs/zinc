//!
//! The circuit `main.zn` file.
//!

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Context;
use serde::Deserialize;

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
