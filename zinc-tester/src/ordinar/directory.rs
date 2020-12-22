//!
//! The integration tests directory.
//!

use std::fs;
use std::path::PathBuf;
use std::process;
use std::sync::Arc;
use std::sync::Mutex;

use anyhow::Context;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use crate::ordinar::project::Project;
use crate::summary::Summary;

///
/// The integration tests directory.
///
#[derive(Debug)]
pub struct Directory {
    /// The project directory paths.
    pub project_paths: Vec<PathBuf>,
}

impl Directory {
    ///
    /// Reads the test directory and stores the project paths located there.
    ///
    pub fn new(path: &PathBuf) -> anyhow::Result<Self> {
        let mut project_paths = Vec::new();
        for entry in fs::read_dir(path)?.into_iter() {
            let entry = entry?;
            let path = entry.path();
            let entry_type = entry
                .file_type()
                .with_context(|| path.to_string_lossy().to_string())?;

            if entry_type.is_dir() {
                project_paths.push(path);
            }
        }
        Ok(Self { project_paths })
    }

    ///
    /// Runs the test projects and writes their results to `summary`.
    ///
    pub fn run(self, verbosity: usize, summary: Arc<Mutex<Summary>>) {
        self.reset_database(verbosity)
            .expect(zinc_const::panic::TEST_DATA_VALID);

        self.project_paths
            .into_par_iter()
            .map(|path| Project::new(verbosity, path).run(summary.clone()))
            .collect::<Vec<anyhow::Result<()>>>();
    }

    ///
    /// Resets the database in order to run the test in a clear environment.
    ///
    fn reset_database(&self, verbosity: usize) -> anyhow::Result<()> {
        process::Command::new(zinc_const::app_name::PSQL)
            .args(if verbosity <= 1 {
                vec!["--quiet"]
            } else {
                vec![]
            })
            .arg("--command")
            .arg("DELETE FROM zandbox.fields;")
            .spawn()
            .with_context(|| zinc_const::app_name::PSQL)?
            .wait()
            .with_context(|| zinc_const::app_name::PSQL)?;
        process::Command::new(zinc_const::app_name::PSQL)
            .args(if verbosity <= 1 {
                vec!["--quiet"]
            } else {
                vec![]
            })
            .arg("--command")
            .arg("DELETE FROM zandbox.contracts;")
            .spawn()
            .with_context(|| zinc_const::app_name::PSQL)?
            .wait()
            .with_context(|| zinc_const::app_name::PSQL)?;
        process::Command::new(zinc_const::app_name::PSQL)
            .args(if verbosity <= 1 {
                vec!["--quiet"]
            } else {
                vec![]
            })
            .arg("--command")
            .arg("DELETE FROM zandbox.projects;")
            .spawn()
            .with_context(|| zinc_const::app_name::PSQL)?
            .wait()
            .with_context(|| zinc_const::app_name::PSQL)?;

        Ok(())
    }
}
