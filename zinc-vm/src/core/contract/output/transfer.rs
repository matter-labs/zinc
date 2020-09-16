//!
//! The virtual machine contract output transfer.
//!

use num_bigint::BigUint;

///
/// The virtual machine contract output transfer.
///
#[derive(Debug)]
pub struct Transfer {
    /// The ID of the token being transferred.
    pub token_id: BigUint,
    /// The sender address.
    pub from: [u8; zinc_const::size::ETH_ADDRESS],
    /// The recepient address.
    pub to: [u8; zinc_const::size::ETH_ADDRESS],
    /// The amount of the tokens being sent.
    pub amount: BigUint,
}

impl Transfer {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        token_id: BigUint,
        from: [u8; zinc_const::size::ETH_ADDRESS],
        to: [u8; zinc_const::size::ETH_ADDRESS],
        amount: BigUint,
    ) -> Self {
        Self {
            token_id,
            from,
            to,
            amount,
        }
    }
}
