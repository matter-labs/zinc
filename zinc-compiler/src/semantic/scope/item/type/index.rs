//!
//! The semantic analyzer scope type item index.
//!

use std::collections::HashMap;
use std::sync::RwLock;

use lazy_static::lazy_static;

use crate::semantic::scope::builtin::BuiltInTypeId;

///
/// The type item index where the unique IDs for all declared types are recorded.
///
/// It is initialized with some built-in and standard library types.
///
/// The type index considers type aliases equal to the type they point to.
///
pub struct Index {
    inner: RwLock<HashMap<usize, String>>,
}

lazy_static! {
    pub static ref INDEX: Index = Index::new();
}

impl Index {
    const INITIAL_CAPACITY: usize = 512;

    pub fn new() -> Self {
        let index = Self {
            inner: RwLock::new(HashMap::with_capacity(Self::INITIAL_CAPACITY)),
        };
        index.next_with_id(
            "function std::crypto::ecc::Point".to_owned(),
            BuiltInTypeId::StdCryptoEccPoint as usize,
        );
        index.next_with_id(
            "function std::crypto::schnorr::Signature".to_owned(),
            BuiltInTypeId::StdCryptoSchnorrSignature as usize,
        );
        index
    }

    pub fn next(&self, title: String) -> usize {
        let mut index = self.inner.write().expect(crate::panic::MUTEX_SYNC);
        let unique_id = index.len();

        log::debug!("Type ID {:04} for {}", unique_id, title);

        index.insert(unique_id, title);
        unique_id
    }

    fn next_with_id(&self, title: String, unique_id: usize) -> usize {
        let mut index = self.inner.write().expect(crate::panic::MUTEX_SYNC);

        log::debug!("Type ID {:04} for {}", unique_id, title);

        index.insert(unique_id, title);
        unique_id
    }
}
