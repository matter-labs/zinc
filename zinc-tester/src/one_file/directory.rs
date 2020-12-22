//!
//! The Zinc tester directory.
//!

use std::convert::TryFrom;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex;

use anyhow::Context;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use crate::error::Error;
use crate::one_file::file::File;
use crate::one_file::metadata::Metadata;
use crate::one_file::runners::IRunnable;
use crate::summary::Summary;

///
/// The integration test directory.
///
#[derive(Debug)]
pub struct Directory {
    /// The directory file paths.
    pub file_paths: Vec<PathBuf>,
}

impl Directory {
    ///
    /// Reads the test directory and stores the test file paths located there.
    ///
    pub fn new(path: &PathBuf) -> anyhow::Result<Self> {
        let directory = fs::read_dir(path)?;
        let mut file_paths = Vec::new();
        for entry in directory.into_iter() {
            let entry = entry?;
            let path = entry.path();
            let entry_type = entry
                .file_type()
                .with_context(|| path.to_string_lossy().to_string())?;

            if entry_type.is_dir() {
                file_paths.extend(
                    Self::new(&path)
                        .with_context(|| path.to_string_lossy().to_string())?
                        .file_paths,
                );
                continue;
            } else if !entry_type.is_file() {
                return Err(Error::InvalidFileType(entry_type))
                    .with_context(|| path.to_string_lossy().to_string());
            }

            let file_extension = path
                .extension()
                .ok_or(Error::GettingFileExtension)
                .with_context(|| path.to_string_lossy().to_string())?;
            if file_extension != zinc_const::extension::SOURCE {
                return Err(Error::InvalidFileExtension(
                    file_extension.to_string_lossy().to_string(),
                ))
                .with_context(|| path.to_string_lossy().to_string());
            }

            file_paths.push(path);
        }
        Ok(Self { file_paths })
    }

    ///
    /// Runs the directory tests and writes their results to `summary`.
    ///
    pub fn run<R: IRunnable>(self, runner: R, summary: Arc<Mutex<Summary>>) {
        self.file_paths
            .into_par_iter()
            .map(|path| {
                let file = File::try_from(&path)
                    .unwrap_or_else(|_| panic!("Test file {:?} is invalid", path));
                let data = Metadata::from_str(file.code.as_str())
                    .unwrap_or_else(|_| panic!("Test file {:?} case data is invalid", path));

                runner
                    .clone()
                    .run(path, file, data, summary.clone())
                    .expect(zinc_const::panic::TEST_DATA_VALID);
            })
            .collect::<Vec<()>>();
    }
}
