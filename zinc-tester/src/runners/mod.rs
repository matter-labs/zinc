//!
//! The test runners.
//!

pub mod evaluation;
pub mod proof_check;

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use crate::data::TestData;
use crate::file::TestFile;
use crate::Summary;

pub trait TestRunner: Sync + Send {
    fn run(
        &self,
        test_file_path: &PathBuf,
        test_file: &TestFile,
        test_data: &TestData,
        summary: Arc<Mutex<Summary>>,
    );
}
