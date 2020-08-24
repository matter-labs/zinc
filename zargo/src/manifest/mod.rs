//!
//! The Zargo manifest.
//!

pub mod project_type;

use std::convert::TryFrom;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use failure::Fail;
use serde_derive::Deserialize;

use crate::manifest::project_type::ProjectType;

///
/// The Zinc project manifest file representation.
///
#[derive(Deserialize)]
pub struct Manifest {
    /// The `project` section.
    pub project: Project,
}

///
/// The Zinc project manifest file `project` section representation.
///
#[derive(Deserialize)]
pub struct Project {
    /// The project name.
    pub name: String,
    /// The project type. See the inner element description.
    pub r#type: ProjectType,
    /// The project version in the string format.
    pub version: String,
}

///
/// The Zinc project manifest file error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// File opening error.
    #[fail(display = "`{}` opening: {}", _0, _1)]
    Opening(String, io::Error),
    /// File metadata getting error.
    #[fail(display = "`{}` metadata: {}", _0, _1)]
    Metadata(String, io::Error),
    /// File reading error.
    #[fail(display = "`{}` reading: {}", _0, _1)]
    Reading(String, io::Error),
    /// File contents parsing error.
    #[fail(display = "`{}` parsing: {}", _0, _1)]
    Parsing(String, toml::de::Error),
    /// File creating error.
    #[fail(display = "`{}` creating: {}", _0, _1)]
    Creating(String, io::Error),
    /// File writing error.
    #[fail(display = "`{}` writing: {}", _0, _1)]
    Writing(String, io::Error),
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
    /// Checks if the manifest exists at the given `path`.
    ///
    pub fn exists_at(path: &PathBuf) -> bool {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(Self::file_name()));
        }
        path.exists()
    }

    ///
    /// Writes the manifest to a file at the given `path`.
    ///
    pub fn write_to(self, path: &PathBuf) -> Result<(), Error> {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(Self::file_name()));
        }

        let mut file =
            File::create(&path).map_err(|error| Error::Creating(Self::file_name(), error))?;
        file.write_all(self.template().as_bytes())
            .map_err(|error| Error::Writing(Self::file_name(), error))
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

        let mut file =
            File::open(path).map_err(|error| Error::Opening(Self::file_name(), error))?;
        let size = file
            .metadata()
            .map_err(|error| Error::Metadata(Self::file_name(), error))?
            .len() as usize;

        let mut buffer = String::with_capacity(size);
        file.read_to_string(&mut buffer)
            .map_err(|error| Error::Reading(Self::file_name(), error))?;

        Ok(toml::from_str(&buffer).map_err(|error| Error::Parsing(Self::file_name(), error))?)
    }
}
