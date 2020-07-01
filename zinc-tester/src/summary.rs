//!
//! The Zinc tester summary.
//!
//!

use std::fmt;
use std::sync::Arc;
use std::sync::Mutex;

///
/// The test results report.
///
#[derive(Debug, Default)]
pub struct Summary {
    /// The passed tests counter.
    pub passed: usize,
    /// The failed tests counter.
    pub failed: usize,
    /// The ignored tests counter.
    pub ignored: usize,
    /// The invalid (with compile errors) tests counter.
    pub invalid: usize,
}

impl Summary {
    ///
    /// Wraps data into a synchronized shared reference.
    ///
    pub fn wrap(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }

    ///
    /// Extracts the data from the synchronized shared reference.
    ///
    pub fn unwrap_arc(summary: Arc<Mutex<Self>>) -> Self {
        Arc::try_unwrap(summary)
            .expect(zinc_const::panic::LAST_SHARED_REFERENCE)
            .into_inner()
            .expect(zinc_const::panic::LAST_SHARED_REFERENCE)
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
