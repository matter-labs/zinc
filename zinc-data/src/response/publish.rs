//!
//! The contract resource POST request.
//!

use serde_derive::Serialize;

///
/// The contract resource POST response body.
///
#[derive(Debug, Serialize)]
pub struct Body {
    /// The contract address.
    pub address: String,
}

impl Body {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(address: String) -> Self {
        Self { address }
    }
}
