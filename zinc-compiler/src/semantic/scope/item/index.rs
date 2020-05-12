//!
//! The semantic analyzer scope item index.
//!

use std::collections::HashMap;
use std::sync::RwLock;

use lazy_static::lazy_static;

///
/// The type item index where the unique IDs for all declared types are recorded.
///
pub struct Index {
    pub inner: RwLock<HashMap<usize, String>>,
}

lazy_static! {
    pub static ref INDEX: Index = Index::new();
}

impl Index {
    const INITIAL_CAPACITY: usize = 1024;

    pub fn new() -> Self {
        Self {
            inner: RwLock::new(HashMap::with_capacity(Self::INITIAL_CAPACITY)),
        }
    }

    pub fn next(&self, title: String) -> usize {
        let mut index = self.inner.write().expect(crate::panic::MUTEX_SYNC);
        let unique_id = index.len();

        log::debug!("Item ID {:04} for {}", unique_id, title);

        index.insert(unique_id, title);
        unique_id
    }
}
