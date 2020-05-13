//!
//! The source code file index.
//!

use std::path::PathBuf;
use std::sync::RwLock;

use lazy_static::lazy_static;

///
/// The global file path array where a `Location` can get the file path by its index.
///
pub struct Index {
    pub inner: RwLock<Vec<PathBuf>>,
}

lazy_static! {
    pub static ref INDEX: Index = Index::new();
}

impl Index {
    const INITIAL_CAPACITY: usize = 64;

    pub fn new() -> Self {
        Self {
            inner: RwLock::new(Vec::with_capacity(Self::INITIAL_CAPACITY)),
        }
    }

    pub fn next(&self, path: &PathBuf) -> usize {
        let mut index = self.inner.write().expect(crate::panic::MUTEX_SYNC);
        let sequence_id = index.len();

        log::debug!("File ID {:06} for {:?}", sequence_id, path);

        index.push(path.to_owned());
        sequence_id
    }

    pub fn get(&self, index: usize) -> PathBuf {
        self.inner
            .read()
            .expect(crate::panic::MUTEX_SYNC)
            .get(index)
            .expect(crate::panic::VALIDATED_DURING_SOURCE_CODE_MAPPING)
            .to_owned()
    }
}
