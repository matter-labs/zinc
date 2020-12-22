//!
//! The Zinc build input file representation.
//!

use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

///
/// The Zinc build input file representation.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Input {
    /// The circuit byte representation.
    Circuit {
        /// The circuit arguments JSON data.
        arguments: serde_json::Value,
    },
    /// The contract byte representation.
    Contract {
        /// The storages JSON data.
        storages: HashMap<String, serde_json::Value>,
        /// The contract input transaction, represented by the `zksync::msg` variable.
        msg: serde_json::Value,
        /// The contract methods arguments JSON data.
        arguments: HashMap<String, serde_json::Value>,
    },
    /// The library byte representation.
    Library,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new_circuit(arguments: serde_json::Value) -> Self {
        Self::Circuit { arguments }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_contract(
        storages: HashMap<String, serde_json::Value>,
        msg: serde_json::Value,
        arguments: HashMap<String, serde_json::Value>,
    ) -> Self {
        Self::Contract {
            storages,
            msg,
            arguments,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_library() -> Self {
        Self::Library
    }
}
