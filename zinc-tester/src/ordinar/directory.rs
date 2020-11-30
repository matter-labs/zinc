//!
//! The Zinc tester directory.
//!

use std::fs;
use std::path::PathBuf;
use std::process;
use std::sync::Arc;
use std::sync::Mutex;

use anyhow::Context;
use colored::Colorize;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use crate::error::Error;
use crate::summary::Summary;

///
/// The integration test directory.
///
#[derive(Debug, PartialEq)]
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
    pub fn run(self, summary: Arc<Mutex<Summary>>) {
        self.project_paths
            .into_par_iter()
            .map(|project_path| -> anyhow::Result<()> {
                let output = process::Command::new(zinc_const::app_name::ZARGO)
                    .arg("build")
                    .arg("--manifest-path")
                    .arg(&project_path)
                    .output()?;
                if !output.status.success() {
                    println!(
                        "[INTEGRATION] {} {}\n{}",
                        "INVALID".red(),
                        project_path.to_string_lossy(),
                        String::from_utf8_lossy(output.stderr.as_slice())
                    );
                    summary
                        .lock()
                        .expect(zinc_const::panic::SYNCHRONIZATION)
                        .invalid += 1;
                    return Ok(());
                }

                let mut datasets_directory_path = project_path.clone();
                datasets_directory_path.push(zinc_const::directory::DATASETS);
                if datasets_directory_path.exists() {
                    let mut original_input_template_file = project_path.clone();
                    original_input_template_file.push(zinc_const::directory::DATA);
                    original_input_template_file.push(format!(
                        "{}.{}",
                        zinc_const::file_name::INPUT,
                        zinc_const::extension::JSON
                    ));

                    for entry in fs::read_dir(datasets_directory_path)?.into_iter() {
                        let entry = entry?;
                        let path = entry.path();

                        let entry_type = entry
                            .file_type()
                            .with_context(|| path.to_string_lossy().to_string())?;
                        if !entry_type.is_file() {
                            return Err(Error::InvalidFileType(entry_type))
                                .with_context(|| path.to_string_lossy().to_string());
                        }
                        let file_extension = path
                            .extension()
                            .ok_or(Error::GettingFileExtension)
                            .with_context(|| path.to_string_lossy().to_string())?;
                        if file_extension != zinc_const::extension::JSON {
                            return Err(Error::InvalidFileExtension(
                                file_extension.to_string_lossy().to_string(),
                            ))
                            .with_context(|| path.to_string_lossy().to_string());
                        }

                        fs::copy(&path, &original_input_template_file)
                            .with_context(|| path.to_string_lossy().to_string())?;

                        let output = process::Command::new(zinc_const::app_name::ZARGO)
                            .arg("test")
                            .arg("--manifest-path")
                            .arg(&project_path)
                            .output()
                            .with_context(|| project_path.to_string_lossy().to_string())?;
                        print!("{}", String::from_utf8_lossy(output.stdout.as_slice()));
                        if !output.status.success() {
                            print!("{}", String::from_utf8_lossy(output.stderr.as_slice()));
                            println!(
                                "[INTEGRATION] {} {} (unit test failure)",
                                "FAILED".bright_red(),
                                path.to_string_lossy(),
                            );
                            summary
                                .lock()
                                .expect(zinc_const::panic::SYNCHRONIZATION)
                                .failed += 1;
                            continue;
                        }

                        println!(
                            "[INTEGRATION] {} {}",
                            "PASSED".green(),
                            path.to_string_lossy(),
                        );
                        summary
                            .lock()
                            .expect(zinc_const::panic::SYNCHRONIZATION)
                            .passed += 1;
                    }
                } else {
                    let output = process::Command::new(zinc_const::app_name::ZARGO)
                        .arg("test")
                        .arg("--manifest-path")
                        .arg(&project_path)
                        .output()
                        .with_context(|| project_path.to_string_lossy().to_string())?;
                    print!("{}", String::from_utf8_lossy(output.stdout.as_slice()));
                    if !output.status.success() {
                        print!("{}", String::from_utf8_lossy(output.stderr.as_slice()));
                        println!(
                            "[INTEGRATION] {} {} (unit test failure)",
                            "FAILED".bright_red(),
                            project_path.to_string_lossy(),
                        );
                        summary
                            .lock()
                            .expect(zinc_const::panic::SYNCHRONIZATION)
                            .failed += 1;
                        return Ok(());
                    }

                    println!(
                        "[INTEGRATION] {} {}",
                        "PASSED".green(),
                        project_path.to_string_lossy(),
                    );
                    summary
                        .lock()
                        .expect(zinc_const::panic::SYNCHRONIZATION)
                        .passed += 1;
                }

                Ok(())
            })
            .collect::<Vec<anyhow::Result<()>>>();
    }
}
