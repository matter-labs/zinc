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
    /// The inner file data storage with the file unique ID as the key.
    pub inner: RwLock<HashMap<usize, Data>>,
}

///
/// The indexed file contents, which are its path and source code text.
///
#[derive(Debug)]
pub struct Data {
    /// The full file path.
    pub path: PathBuf,
    /// The file contents as string.
    pub code: String,
}

lazy_static! {
    pub static ref INDEX: Index = Index::new();
}

impl Index {
    /// The default file index capacity.
    const INITIAL_CAPACITY: usize = 64;

    ///
    /// Initializes an index instance.
    ///
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(HashMap::with_capacity(Self::INITIAL_CAPACITY)),
        }
    }

    ///
    /// Allocates the next file sequence ID.
    ///
    pub fn next(&self, path: &PathBuf, code: String) -> usize {
        let mut index = self
            .inner
            .write()
            .expect(zinc_const::panic::SYNCHRONIZATION);
        let sequence_id = index.len() + 1;

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

    ///
    /// Returns the current file sequence ID.
    ///
    pub fn current(&self) -> usize {
        self.inner
            .read()
            .expect(zinc_const::panic::SYNCHRONIZATION)
            .len()
    }

    ///
    /// Peeks the next file sequence ID.
    ///
    pub fn peek(&self) -> usize {
        self.inner
            .read()
            .expect(zinc_const::panic::SYNCHRONIZATION)
            .len()
            + 1
    }

    ///
    /// Get the file path by its index.
    ///
    pub fn get_path(&self, index: usize) -> PathBuf {
        self.inner
            .read()
            .expect(zinc_const::panic::SYNCHRONIZATION)
            .get(&index)
            .expect(zinc_const::panic::VALIDATED_DURING_SOURCE_CODE_MAPPING)
            .path
            .to_owned()
    }
}
