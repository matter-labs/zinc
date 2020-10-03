//!
//! The contract resource `initialize` POST request.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use zksync_types::AccountId;

///
/// The contract resource `initialize` POST response body.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    /// The zkSync account ID.
    pub account_id: AccountId,
}

impl Body {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(account_id: AccountId) -> Self {
        Self { account_id }
    }
}
