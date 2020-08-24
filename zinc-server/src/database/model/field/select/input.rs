//!
//! The database contract storage field SELECT input model.
//!

///
/// The database contract storage field SELECT input model.
///
pub struct Input {
    /// The contract account ID referencing `contracts.account_id`.
    pub account_id: i64,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(account_id: i64) -> Self {
        Self { account_id }
    }
}
