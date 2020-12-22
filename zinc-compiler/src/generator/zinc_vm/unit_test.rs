//!
//! The bytecode unit test.
//!

///
/// Unit test metadata.
///
#[derive(Debug)]
pub struct UnitTest {
    /// The entry function type unique ID.
    pub type_id: usize,
    /// The unit test name.
    pub name: String,
    /// Whether the test should fail to be successful.
    pub should_panic: bool,
    /// Whether the test is marked as ignored.
    pub is_ignored: bool,
    /// The optional transaction variable.
    pub zksync_msg: Option<zinc_types::TransactionMsg>,
}

impl UnitTest {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        type_id: usize,
        name: String,
        should_panic: bool,
        is_ignored: bool,
        zksync_msg: Option<zinc_types::TransactionMsg>,
    ) -> Self {
        Self {
            type_id,
            name,
            should_panic,
            is_ignored,
            zksync_msg,
        }
    }
}
