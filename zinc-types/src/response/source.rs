//!
//! The project resource GET `source` response.
//!

use serde::Deserialize;
use serde::Serialize;

///
/// The project resource GET `source` response body.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    /// The project compiler version.
    pub zinc_version: String,
    /// The project data.
    pub project: zinc_project::Project,
}

impl Body {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(zinc_version: String, project: zinc_project::Project) -> Self {
        Self {
            zinc_version,
            project,
        }
    }
}
