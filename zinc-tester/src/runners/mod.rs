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

pub trait Runnable: Clone + Sync + Send {
    fn run(self, path: PathBuf, file: File, metadata: Metadata, summary: Arc<Mutex<Summary>>);
}
