//!
//! The Zinc VM bytecode circuit application unit test.
//!

use serde::Deserialize;
use serde::Serialize;

///
/// The circuit unit test.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitTest {
    /// The unit test address in the bytecode.
    pub address: usize,
    /// If an error means success, is set by the `#[should_panic]` macro
    pub should_panic: bool,
    /// If the test must be ignored, is set by the `#[ignore]` macro
    pub is_ignored: bool,
}

impl UnitTest {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(address: usize, should_panic: bool, is_ignored: bool) -> Self {
        Self {
            address,
            should_panic,
            is_ignored,
        }
    }
}
