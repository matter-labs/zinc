//!
//! The database project SELECT source model.
//!

///
/// The database project SELECT source input model.
///
#[derive(Debug)]
pub struct Input {
    /// The project name.
    pub name: String,
    /// The project version.
    pub version: semver::Version,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(name: String, version: semver::Version) -> Self {
        Self { name, version }
    }
}

///
/// The database project SELECT source output model.
///
#[derive(Debug, sqlx::FromRow)]
pub struct Output {
    /// The project name.
    pub name: String,
    /// The project version.
    pub version: String,

    /// The project compiler version.
    pub zinc_version: String,
    /// The project JSON representation.
    pub project: serde_json::Value,
}
