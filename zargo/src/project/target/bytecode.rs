//!
//! The bytecode binary file.
//!

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use anyhow::Context;

///
/// The bytecode binary file representation.
///
pub struct Bytecode {
    /// The file contents.
    pub inner: Vec<u8>,
}

impl Bytecode {
    ///
    /// Reads the bytecode from the project at `path`.
    ///
    pub fn try_from_path(path: &PathBuf, is_release: bool) -> anyhow::Result<Self> {
        let target = if is_release {
            zinc_const::directory::TARGET_RELEASE
        } else {
            zinc_const::directory::TARGET_DEBUG
        };

        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(target) {
                path.push(PathBuf::from(target));
            }
            path.push(PathBuf::from(Self::file_name()));
        }

        let mut file = File::open(&path).with_context(|| path.to_string_lossy().to_string())?;
        let size = file
            .metadata()
            .with_context(|| path.to_string_lossy().to_string())?
            .len() as usize;

        let mut buffer = Vec::with_capacity(size);
        file.read_to_end(&mut buffer)
            .with_context(|| path.to_string_lossy().to_string())?;

        Ok(Self { inner: buffer })
    }

    ///
    /// Creates a string with the default file name.
    ///
    fn file_name() -> String {
        zinc_const::file_name::BINARY.to_owned()
    }
}
