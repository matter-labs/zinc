//!
//! The Zinc server shared application data.
//!

use std::sync::Arc;
use std::sync::RwLock;

///
/// The Zinc server shared application data.
///
#[derive(Clone)]
pub struct AppData {
    pub count: u32,
}

impl AppData {
    ///
    /// A shortcut constructor.
    ///
    pub fn new() -> Self {
        Self { count: 42 }
    }

    ///
    /// Wraps the data into `Arc<Mutex<_>>`.
    ///
    pub fn wrap(self) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(self))
    }
}
