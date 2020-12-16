//!
//! The Zinc project representation.
//!

pub mod r#type;

use serde::Deserialize;
use serde::Serialize;

use crate::manifest::Manifest;
use crate::source::Source;

///
/// The Zinc project representation.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    /// The project manifest.
    pub manifest: Manifest,
    /// The project source code.
    pub source: Source,
}

impl Project {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(manifest: Manifest, source: Source) -> Self {
        Self { manifest, source }
    }
}
