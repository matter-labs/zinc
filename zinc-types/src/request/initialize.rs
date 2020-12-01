//!
//! The contract resource `initialize` POST request.
//!

use std::iter::IntoIterator;

use serde::Deserialize;
use serde::Serialize;

use zksync_types::Address;

use crate::transaction::Transaction;

///
/// The contract resource `initialize` POST request query.
///
#[derive(Debug, Deserialize)]
pub struct Query {
    /// The contract ETH address.
    pub address: Address,
}

impl Query {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(address: Address) -> Self {
        Self { address }
    }
}

impl IntoIterator for Query {
    type Item = (&'static str, String);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![(
            "address",
            serde_json::to_string(&self.address)
                .expect(zinc_const::panic::DATA_CONVERSION)
                .replace("\"", ""),
        )]
        .into_iter()
    }
}

///
/// The contract resource `initialize` POST request body.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    /// The signed initial transfer transaction which must be sent directly to zkSync.
    pub transaction: Transaction,
}

impl Body {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(transaction: Transaction) -> Self {
        Self { transaction }
    }
}
