//!
//! The Zinc tester summary.
//!
//!

use std::fmt;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug, Default)]
pub struct Summary {
    pub passed: usize,
    pub failed: usize,
    pub ignored: usize,
    pub invalid: usize,
}

impl Summary {
    pub fn wrap(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }

    pub fn unwrap_arc(summary: Arc<Mutex<Self>>) -> Self {
        Arc::try_unwrap(summary)
            .expect(crate::panic::LAST_SHARED_REFERENCE)
            .into_inner()
            .expect(crate::panic::LAST_SHARED_REFERENCE)
    }
}

impl fmt::Display for Summary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} passed, {} failed, {} ignored, {} invalid",
            self.passed, self.failed, self.ignored, self.invalid
        )
    }
}
