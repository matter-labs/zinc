//!
//! The Zinc project manifest file.
//!

use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Context;
use serde::Deserialize;
use serde::Serialize;

use crate::project::r#type::Type as ProjectType;

///
/// The Zinc project manifest file representation.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Manifest {
    /// The `project` section.
    pub project: Project,
    /// The `dependencies` section.
    pub dependencies: Option<HashMap<String, semver::Version>>,
}

///
/// The `project` section representation.
///
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Project {
    /// The project name.
    pub name: String,
    /// The project type. See the inner element description.
    pub r#type: ProjectType,
    /// The project version in the string format.
    pub version: semver::Version,
}

impl Project {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(name: String, r#type: ProjectType, version: semver::Version) -> Self {
        Self {
            name,
            r#type,
            version,
        }
    }
}

impl Manifest {
    ///
    /// Creates a new manifest instance.
    ///
    pub fn new(project_name: &str, project_type: ProjectType) -> Self {
        Self {
            project: Project {
                name: project_name.to_owned(),
                r#type: project_type,
                version: semver::Version::new(0, 1, 0),
            },
            dependencies: Some(HashMap::new()),
        }
    }

    ///
    /// Checks if the manifest exists in the project at the given `path`.
    ///
    pub fn exists_at(path: &PathBuf) -> bool {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(Self::file_name()));
        }
        path.exists()
    }

    ///
    /// Writes the manifest to a file in the project at the given `path`.
    ///
    pub fn write_to(&self, path: &PathBuf) -> anyhow::Result<()> {
        let mut path = path.to_owned();
        if path.is_dir() || !path.ends_with(Self::file_name()) {
            path.push(PathBuf::from(Self::file_name()));
        }

        let mut file = File::create(&path).with_context(|| path.to_string_lossy().to_string())?;
        file.write_all(
            toml::to_string_pretty(self)
                .expect(zinc_const::panic::DATA_CONVERSION)
                .as_bytes(),
        )
        .with_context(|| path.to_string_lossy().to_string())?;

        Ok(())
    }

    ///
    /// Creates a string with the default file name.
    ///
    fn file_name() -> String {
        format!(
            "{}.{}",
            zinc_const::file_name::MANIFEST,
            zinc_const::extension::MANIFEST
        )
    }
}

impl TryFrom<&PathBuf> for Manifest {
    type Error = anyhow::Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(Self::file_name()));
        }

        let mut file = File::open(&path).with_context(|| path.to_string_lossy().to_string())?;
        let size = file
            .metadata()
            .with_context(|| path.to_string_lossy().to_string())?
            .len() as usize;

        let mut buffer = String::with_capacity(size);
        file.read_to_string(&mut buffer)
            .with_context(|| path.to_string_lossy().to_string())?;

        Ok(toml::from_str(buffer.as_str()).with_context(|| path.to_string_lossy().to_string())?)
    }
}
