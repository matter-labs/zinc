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

pub static FILE_NAME_DEFAULT: &str = "Zargo.toml";
pub static PROJECT_VERSION_DEFAULT: &str = "0.1.0";

#[derive(Deserialize)]
pub struct Manifest {
    pub project: Project,
}

#[derive(Deserialize)]
pub struct Project {
    pub name: String,
    pub r#type: ProjectType,
    pub version: String,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "`{}` opening: {}", _0, _1)]
    Opening(&'static str, io::Error),
    #[fail(display = "`{}` metadata: {}", _0, _1)]
    Metadata(&'static str, io::Error),
    #[fail(display = "`{}` reading: {}", _0, _1)]
    Reading(&'static str, io::Error),
    #[fail(display = "`{}` parsing: {}", _0, _1)]
    Parsing(&'static str, toml::de::Error),
    #[fail(display = "`{}` creating: {}", _0, _1)]
    Creating(&'static str, io::Error),
    #[fail(display = "`{}` writing: {}", _0, _1)]
    Writing(&'static str, io::Error),
}

impl Manifest {
    pub fn new(project_name: &str, project_type: ProjectType) -> Self {
        Self {
            project: Project {
                name: project_name.to_owned(),
                r#type: project_type,
                version: PROJECT_VERSION_DEFAULT.to_owned(),
            },
        }
    }

    pub fn exists_at(path: &PathBuf) -> bool {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(FILE_NAME_DEFAULT));
        }
        path.exists()
    }

    pub fn write_to(self, path: &PathBuf) -> Result<(), Error> {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(FILE_NAME_DEFAULT));
        }

        let mut file =
            File::create(&path).map_err(|error| Error::Creating(FILE_NAME_DEFAULT, error))?;
        file.write_all(self.template().as_bytes())
            .map_err(|error| Error::Writing(FILE_NAME_DEFAULT, error))
    }

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
}

impl TryFrom<&PathBuf> for Manifest {
    type Error = Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(FILE_NAME_DEFAULT));
        }

        let mut file =
            File::open(path).map_err(|error| Error::Opening(FILE_NAME_DEFAULT, error))?;
        let size = file
            .metadata()
            .map_err(|error| Error::Metadata(FILE_NAME_DEFAULT, error))?
            .len() as usize;

        let mut buffer = String::with_capacity(size);
        file.read_to_string(&mut buffer)
            .map_err(|error| Error::Reading(FILE_NAME_DEFAULT, error))?;

        Ok(toml::from_str(&buffer).map_err(|error| Error::Parsing(FILE_NAME_DEFAULT, error))?)
    }
}
