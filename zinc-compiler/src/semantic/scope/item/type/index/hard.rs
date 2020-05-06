//!
//! The semantic analyzer scope type item index.
//!

use std::collections::HashMap;
use std::sync::RwLock;

///
/// The type item index where the unique IDs for all declared types are recorded.
///
pub struct Index {
    inner: RwLock<HashMap<usize, String>>,
}

impl Index {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
        }
    }

    pub fn next(&self, title: String) -> usize {
        let mut index = self.inner.write().expect(crate::panic::MUTEX_SYNC);
        let unique_id = index.len();
        index.insert(unique_id, title);
        unique_id
    }
}
