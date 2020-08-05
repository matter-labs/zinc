//!
//! The PostgreSQL program instance model.
//!

///
/// The PostgreSQL program instance model.
///
pub struct Instance {
    /// The program ID referencing `programs.id`.
    pub program_id: u32,

    /// The contract instance owner address.
    pub owner_address: [char; 40],
}

impl Instance {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(program_id: u32, owner_address: [char; 40]) -> Self {
        Self {
            program_id,

            owner_address,
        }
    }
}
