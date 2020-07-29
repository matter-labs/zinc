//!
//! The Zinc VM bytecode unit test.
//!

///
/// Unit test metadata.
///
#[derive(Debug)]
pub struct Test {
    /// The unit test name.
    pub name: String,
    /// Whether the test should fail to be successful.
    pub should_panic: bool,
    /// Whether the test is marked as ignored.
    pub is_ignored: bool,
}

impl Test {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(name: String, should_panic: bool, is_ignored: bool) -> Self {
        Self {
            name,
            should_panic,
            is_ignored,
        }
    }
}
