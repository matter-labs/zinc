//!
//! The Zinc tester directory.
//!

pub mod error;

use std::convert::TryFrom;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use serde::Deserialize;

use crate::file::File;
use crate::metadata::Metadata;
use crate::runners::IRunnable;
use crate::summary::Summary;

use self::error::Error;

///
/// The integration test directory.
///
#[derive(Debug, Deserialize, PartialEq)]
pub struct Directory {
    /// The directory file paths.
    pub file_paths: Vec<PathBuf>,
}

impl Directory {
    ///
    /// Reads the test directory and stores the test file paths located there.
    ///
    pub fn new(path: &PathBuf) -> Result<Self, Error> {
        let directory = fs::read_dir(path).map_err(Error::Reading)?;
        let mut file_paths = Vec::new();
        for entry in directory.into_iter() {
            let entry = entry.map_err(Error::GettingFileEntry)?;
            let entry_path = entry.path();

            let entry_type = entry.file_type().map_err(|error| {
                Error::GettingFileType(entry_path.as_os_str().to_owned(), error)
            })?;

            if entry_type.is_dir() {
                file_paths.extend(Self::new(&entry_path)?.file_paths);
                continue;
            } else if !entry_type.is_file() {
                return Err(Error::InvalidFileType(
                    entry_path.as_os_str().to_owned(),
                    entry_type,
                ));
            }

            let file_extension = entry_path
                .extension()
                .ok_or_else(|| Error::GettingFileExtension(entry_path.as_os_str().to_owned()))?;
            if file_extension != zinc_const::extension::SOURCE {
                return Err(Error::InvalidFileExtension(
                    entry_path.as_os_str().to_owned(),
                    file_extension.to_owned(),
                ));
            }

            file_paths.push(entry_path);
        }
        Ok(Self { file_paths })
    }

    ///
    /// Runs the directory tests and returns their result summary.
    ///
    pub fn run<R: IRunnable>(self, runner: R) -> Summary {
        let summary = Summary::default().wrap();

        self.file_paths
            .into_par_iter()
            .map(|path| {
                let file = File::try_from(&path)
                    .unwrap_or_else(|_| panic!("Test file {:?} is invalid", path));
                let data = Metadata::from_str(file.code.as_str())
                    .unwrap_or_else(|_| panic!("Test file {:?} case data is invalid", path));

                runner.clone().run(path, file, data, summary.clone());
            })
            .collect::<Vec<()>>();

        Summary::unwrap_arc(summary)
    }
}
