//!
//! The Zargo `new` command.
//!

use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io;

use failure::Fail;
use structopt::StructOpt;
use std::io::Write;

#[derive(Debug, StructOpt)]
pub struct Command {
    #[structopt(
        long = "name",
        name = "NAME",
        help = "Set the resulting project name, defaults to the directory name",
    )]
    name: Option<String>,
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "root directory creating: {}", _0)]
    CreatingRootDirectory(io::Error),
    #[fail(display = "source directory creating: {}", _0)]
    CreatingSourceDirectory(io::Error),
    #[fail(display = "source `main.zn` file creating: {}", _0)]
    CreatingSourceMainFile(io::Error),
    #[fail(display = "source `main.zn` file template writing: {}", _0)]
    WritingSourceMainFileTemplate(io::Error),
    #[fail(display = "project name is missing and cannot be inferred")]
    ProjectNameInvalid,
}

impl Command {
    pub fn execute(mut self) -> Result<(), Error> {
        let project_name = self.name.take().unwrap_or(self.path.file_stem().ok_or(Error::ProjectNameInvalid)?.to_string_lossy().to_string());

        fs::create_dir_all(&self.path).map_err(Error::CreatingRootDirectory)?;

        let mut source_directory = self.path.clone();
        source_directory.push(PathBuf::from("src"));
        fs::create_dir_all(source_directory).map_err(Error::CreatingSourceDirectory)?;

        let mut source_main_file = self.path.clone();
        source_main_file.push(PathBuf::from("src/main.zn"));
        let mut main_file = File::create(source_main_file).map_err(Error::CreatingSourceMainFile)?;
        main_file.write_all(self.template(&project_name).as_bytes()).map_err(Error::WritingSourceMainFileTemplate)?;

        log::info!("The empty project '{}' has been created at {}", project_name, self.path.to_string_lossy());
        Ok(())
    }

    fn template(&self, project_name: &str) -> String {
        format!(r#"//!
//! The '{}' main module.
//!

fn main() -> u8 {{
    dbg!("Zello, World!");
    42
}}
"#, project_name)
    }
}
