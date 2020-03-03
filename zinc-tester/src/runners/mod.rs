//!
//! The test runners.
//!

mod evaluation;
mod proof_check;

pub use self::evaluation::EvaluationTestRunner;
pub use self::proof_check::ProofCheckRunner;
pub use crate::Summary;

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use crate::data::TestData;
use crate::file::TestFile;

pub trait TestRunner: Sync + Send {
    fn run(
        &self,
        test_file_path: &PathBuf,
        test_file: &TestFile,
        test_data: &TestData,
        summary: Arc<Mutex<Summary>>,
    );
}
