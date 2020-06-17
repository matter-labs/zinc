//!
//! The source code file index.
//!

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::RwLock;

use lazy_static::lazy_static;

///
/// The global file index where a `Location` instance can get the file contents by its index.
///
#[derive(Debug)]
pub struct Index {
    pub inner: RwLock<HashMap<usize, Data>>,
}

///
/// The indexed file contents, which are its path and source code text.
///
#[derive(Debug)]
pub struct Data {
    pub path: PathBuf,
    pub code: String,
}

lazy_static! {
    pub static ref INDEX: Index = Index::new();
}

impl Index {
    const INITIAL_CAPACITY: usize = 64;

    pub fn new() -> Self {
        Self {
            inner: RwLock::new(HashMap::with_capacity(Self::INITIAL_CAPACITY)),
        }
    }

    pub fn next(&self, path: &PathBuf, code: String) -> usize {
        let mut index = self.inner.write().expect(crate::panic::MUTEX_SYNC);
        let sequence_id = index.len();

        log::debug!("File ID {:06} for {:?}", sequence_id, path);

        index.insert(
            sequence_id,
            Data {
                path: path.to_owned(),
                code,
            },
        );
        sequence_id
    }

    pub fn get_path(&self, index: usize) -> PathBuf {
        self.inner
            .read()
            .expect(crate::panic::MUTEX_SYNC)
            .get(&index)
            .expect(crate::panic::VALIDATED_DURING_SOURCE_CODE_MAPPING)
            .path
            .to_owned()
    }
}
