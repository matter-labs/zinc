//!
//! The test runners.
//!

pub mod evaluation;
pub mod proof_check;

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use crate::file::File;
use crate::metadata::Metadata;
use crate::Summary;

///
/// The test runner trait.
///
/// There are several implementations for different testing intensiveness.
///
pub trait IRunnable: Clone + Sync + Send {
    ///
    /// Runs a test and writes its result to `summary`.
    ///
    fn run(self, path: PathBuf, file: File, metadata: Metadata, summary: Arc<Mutex<Summary>>);
}
