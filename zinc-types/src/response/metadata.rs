//!
//! The project resource GET `metadata` response.
//!

use serde::Deserialize;
use serde::Serialize;

///
/// The project resource GET `metadata` response body.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    /// The project metadata.
    pub projects: Vec<zinc_project::ManifestProject>,
}

impl Body {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(projects: Vec<zinc_project::ManifestProject>) -> Self {
        Self { projects }
    }
}
