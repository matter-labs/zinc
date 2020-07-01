//!
//! The source code directory string representation.
//!

use std::collections::HashMap;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::source::string::String as SourceString;

///
/// The Zinc virtual source code directory, which consists of its name and virtual files.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct String {
    /// The virtual directory subpath.
    pub path: ::std::string::String,
    /// The virtual dependency files.
    pub modules: HashMap<::std::string::String, SourceString>,
}
