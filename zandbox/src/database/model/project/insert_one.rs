//!
//! The database project INSERT one model.
//!

///
/// The database project INSERT one input model.
///
#[derive(Debug)]
pub struct Input {
    /// The project name.
    pub name: String,
    /// The project version.
    pub version: semver::Version,

    /// The project compiler version.
    pub zinc_version: semver::Version,
    /// The project tree JSON representation.
    pub project: zinc_project::Project,
    /// The project bytecode.
    pub bytecode: Vec<u8>,
    /// The project verifying key as a byte array.
    pub verifying_key: Vec<u8>,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        name: String,
        version: semver::Version,

        zinc_version: semver::Version,
        project: zinc_project::Project,
        bytecode: Vec<u8>,
        verifying_key: Vec<u8>,
    ) -> Self {
        Self {
            name,
            version,

            zinc_version,
            project,
            bytecode,
            verifying_key,
        }
    }
}
