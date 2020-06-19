//!
//! The Zinc VM bytecode unit test.
//!

///
/// Unit test metadata.
///
#[derive(Debug)]
pub struct Test {
    pub name: String,
    pub should_panic: bool,
    pub is_ignored: bool,
}

impl Test {
    pub fn new(name: String, should_panic: bool, is_ignored: bool) -> Self {
        Self {
            name,
            should_panic,
            is_ignored,
        }
    }
}
