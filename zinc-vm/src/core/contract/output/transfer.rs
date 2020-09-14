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
    token_id: BigUint,
    /// The sender address.
    from: Vec<u8>,
    /// The recepient address.
    to: Vec<u8>,
    /// The amount of the tokens being sent.
    amount: BigUint,
    /// The transaction fee.
    fee: BigUint,
}

impl Transfer {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        token_id: BigUint,
        from: Vec<u8>,
        to: Vec<u8>,
        amount: BigUint,
        fee: BigUint,
    ) -> Self {
        Self {
            token_id,
            from,
            to,
            amount,
            fee,
        }
    }
}
