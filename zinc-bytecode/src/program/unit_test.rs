//!
//! The Zinc VM bytecode unit test program.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

///
/// The unit test data, which is attached to a program, if it is marked as a unit test.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitTest {
    /// The name of the unit test function
    pub name: String,
    /// If an error means success, is set by the `#[should_panic]` macro
    pub should_panic: bool,
    /// If the test must be ignored, is set by the `#[ignore]` macro
    pub is_ignored: bool,
}

impl UnitTest {
    ///
    /// Creates a unit test data instance.
    ///
    pub fn new(name: String, should_panic: bool, is_ignored: bool) -> Self {
        Self {
            name,
            should_panic,
            is_ignored,
        }
    }
}
