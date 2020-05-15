//!
//! The compiler executable.
//!

use std::io;
use std::path::PathBuf;
use std::process;
use std::process::ExitStatus;

use failure::Fail;

pub struct Compiler {}

static BINARY_NAME_DEFAULT: &str = "znc";

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "spawning: {}", _0)]
    Spawning(io::Error),
    #[fail(display = "waiting: {}", _0)]
    Waiting(io::Error),
    #[fail(display = "failure: {}", _0)]
    Failure(ExitStatus),
}

impl Compiler {
    pub fn build(
        verbosity: usize,
        data_path: &PathBuf,
        build_path: &PathBuf,
        source_path: &PathBuf,
    ) -> Result<(), Error> {
        let mut child = process::Command::new(BINARY_NAME_DEFAULT)
            .args(vec!["-v"; verbosity])
            .arg("--data")
            .arg(data_path)
            .arg("--build")
            .arg(build_path)
            .arg(source_path)
            .spawn()
            .map_err(Error::Spawning)?;

        let status = child.wait().map_err(Error::Waiting)?;

        if !status.success() {
            return Err(Error::Failure(status));
        }

        Ok(())
    }
}
