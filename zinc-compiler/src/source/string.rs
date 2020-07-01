//!
//! The source code string representation.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::source::directory::string::String as DirectoryString;
use crate::source::file::string::String as FileString;

///
/// The string source code representation.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum String {
    /// The virtual file string data.
    File(FileString),
    /// The virtual directory string data.
    Directory(DirectoryString),
}
