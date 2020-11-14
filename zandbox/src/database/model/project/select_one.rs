//!
//! The database project SELECT one model.
//!

///
/// The database project SELECT one input model.
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
/// The database project SELECT one output model.
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
    /// The project bytecode.
    pub bytecode: Vec<u8>,
    /// The project verifying key.
    pub verifying_key: Vec<u8>,
}
