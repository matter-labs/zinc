//!
//! The source code file string representation.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

///
/// The Zinc virtual source code file, which consists of its name and source code string.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct String {
    /// The virtual file subpath.
    pub path: ::std::string::String,
    /// The source code string data.
    pub code: ::std::string::String,
}

impl String {
    ///
    /// Checks whether the file is the entry point.
    ///
    pub fn is_entry(&self) -> bool {
        self.is_application_entry() || self.is_module_entry()
    }

    ///
    /// Checks whether the file is the application entry point.
    ///
    pub fn is_application_entry(&self) -> bool {
        self.path.as_str() == zinc_const::file_name::APPLICATION_ENTRY
    }

    ///
    /// Checks whether the file is the module entry point.
    ///
    pub fn is_module_entry(&self) -> bool {
        self.path.as_str() == zinc_const::file_name::MODULE_ENTRY
    }
}
