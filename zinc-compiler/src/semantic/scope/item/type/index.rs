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
/// The index treats type aliases equal to the type they point to.
///
pub struct Index {
    pub inner: RwLock<HashMap<usize, String>>,
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
            "structure std::crypto::ecc::Point".to_owned(),
            BuiltInTypeId::StdCryptoEccPoint as usize,
        );
        index.next_with_id(
            "structure std::crypto::schnorr::Signature".to_owned(),
            BuiltInTypeId::StdCryptoSchnorrSignature as usize,
        );
        index
    }

    pub fn next(&self, title: String) -> usize {
        let type_id = self
            .inner
            .write()
            .expect(zinc_const::panic::MUTEX_SYNC)
            .len();

        self.next_with_id(title, type_id)
    }

    fn next_with_id(&self, title: String, type_id: usize) -> usize {
        let mut index = self.inner.write().expect(zinc_const::panic::MUTEX_SYNC);

        log::debug!("Type ID {:06} for {}", type_id, title);

        index.insert(type_id, title);
        type_id
    }
}
