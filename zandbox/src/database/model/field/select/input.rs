//!
//! The database contract storage field SELECT input model.
//!

///
/// The database contract storage field SELECT input model.
///
pub struct Input {
    /// The contract account ID referencing `contracts.contract_id`.
    pub contract_id: i64,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(contract_id: i64) -> Self {
        Self { contract_id }
    }
}
