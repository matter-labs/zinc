//!
//! The test runners.
//!

pub mod evaluation;

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use crate::one_file::file::File;
use crate::one_file::metadata::Metadata;
use crate::summary::Summary;

///
/// The test runner trait.
///
/// There are several implementations for different testing intensiveness.
///
pub trait IRunnable: Clone + Sync + Send {
    ///
    /// Runs a test and writes its result to `summary`.
    ///
    fn run(
        self,
        path: PathBuf,
        file: File,
        metadata: Metadata,
        summary: Arc<Mutex<Summary>>,
    ) -> anyhow::Result<()>;
}
