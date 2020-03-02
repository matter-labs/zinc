mod evaluation;
mod proof_check;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::data::TestData;
use crate::file::TestFile;

pub use crate::Summary;
pub use evaluation::EvaluationTestRunner;
pub use proof_check::ProofCheckRunner;

pub trait TestRunner: Sync + Send {
    fn run(
        &self,
        test_file_path: &PathBuf,
        test_file: &TestFile,
        test_data: &TestData,
        summary: Arc<Mutex<Summary>>
    );
}

