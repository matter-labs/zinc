//!
//! The contract resource `fee` PUT request.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use num_old::BigUint;

///
/// The contract resource `fee` PUT response body.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    /// The zkSync account ID.
    pub fee: BigUint,
}

impl Body {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(fee: BigUint) -> Self {
        Self { fee }
    }
}
