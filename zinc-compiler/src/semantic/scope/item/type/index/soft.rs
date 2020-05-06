//!
//! The semantic analyzer scope type item index.
//!

use std::collections::HashMap;
use std::sync::RwLock;

use crate::semantic::scope::builtin::BuiltInTypeId;

///
/// The type item index where the unique IDs for all declared types are recorded.
///
/// It is initialized with some built-in and standard library types.
///
/// The soft index considers type aliases equal to the type they point to.
///
pub struct Index {
    inner: RwLock<HashMap<usize, String>>,
}

impl Index {
    pub fn new() -> Self {
        let mut index = HashMap::with_capacity(BuiltInTypeId::Count as usize);
        index.insert(
            BuiltInTypeId::StdCryptoEccPoint as usize,
            "struct std::crypto::ecc::Point".to_owned(),
        );
        index.insert(
            BuiltInTypeId::StdCryptoSchnorrSignature as usize,
            "struct std::crypto::schnorr::Signature".to_owned(),
        );

        Self {
            inner: RwLock::new(index),
        }
    }

    pub fn next(&self, title: String) -> usize {
        let mut index = self.inner.write().expect(crate::panic::MUTEX_SYNC);
        let unique_id = index.len();
        index.insert(unique_id, title);
        unique_id
    }
}
