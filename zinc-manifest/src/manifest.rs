//!
//! The Zinc project manifest file.
//!

use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use serde::Deserialize;

use crate::error::Error;
use crate::project_type::ProjectType;

///
/// The Zinc project manifest file representation.
///
#[derive(Debug, Deserialize)]
pub struct Manifest {
    /// The `project` section.
    pub project: Project,
}

///
/// The `project` section representation.
///
#[derive(Debug, Deserialize)]
pub struct Project {
    /// The project name.
    pub name: String,
    /// The project type. See the inner element description.
    pub r#type: ProjectType,
    /// The project version in the string format.
    pub version: String,
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
                version: zinc_const::zargo::INITIAL_PROJECT_VERSION.to_owned(),
            },
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
    pub fn write_to(self, path: &PathBuf) -> crate::Result<()> {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(Self::file_name()));
        }

        let mut file = File::create(&path)?;
        file.write_all(self.template().as_bytes())?;

        Ok(())
    }

    ///
    /// The manifest `*.toml` file template function.
    ///
    fn template(&self) -> String {
        format!(
            r#"[project]
name = "{}"
type = "{}"
version = "{}"
"#,
            self.project.name, self.project.r#type, self.project.version,
        )
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
    type Error = Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(Self::file_name()));
        }

        let mut file = File::open(&path)?;
        let size = file.metadata()?.len() as usize;

        let mut buffer = String::with_capacity(size);
        file.read_to_string(&mut buffer)?;

        Ok(toml::from_str(buffer.as_str())?)
    }
}
