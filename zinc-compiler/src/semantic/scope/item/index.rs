//!
//! The semantic analyzer scope item index.
//!

use std::collections::HashMap;
use std::sync::RwLock;

use lazy_static::lazy_static;

///
/// The item index where the unique IDs for all declared items are recorded.
///
/// The index treats item aliases (`crate`, `super`, `self`) equal to the item they point to.
///
pub struct Index {
    /// The inner item storage with the item unique ID as the key.
    pub inner: RwLock<HashMap<usize, String>>,
}

lazy_static! {
    pub static ref INDEX: Index = Index::new();
}

impl Index {
    /// The item hashmap default capacity.
    const INITIAL_CAPACITY: usize = 1024;

    ///
    /// A shortcut constructor.
    ///
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(HashMap::with_capacity(Self::INITIAL_CAPACITY)),
        }
    }

    ///
    /// Generate the next item sequence ID and add the ID with the item `title` to the index.
    ///
    pub fn next(&self, title: String) -> usize {
        let mut index = self
            .inner
            .write()
            .expect(zinc_const::panic::SYNCHRONIZATION);
        let item_id = index.len();

        log::debug!("Item ID {:06} for {}", item_id, title);

        index.insert(item_id, title);
        item_id
    }
}
