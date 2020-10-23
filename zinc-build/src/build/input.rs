//!
//! The Zinc build input file representation.
//!

use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value as JsonValue;

///
/// The Zinc build input file representation.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Input {
    /// The circuit byte representation.
    Circuit {
        /// The circuit arguments JSON data.
        arguments: JsonValue,
    },
    /// The contract byte representation.
    Contract {
        /// The storage file JSON data.
        storage: JsonValue,
        /// The contract input transaction, represented by the `zksync::msg` variable.
        msg: JsonValue,
        /// The contract methods arguments JSON data.
        arguments: HashMap<String, JsonValue>,
    },
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new_circuit(arguments: JsonValue) -> Self {
        Self::Circuit { arguments }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_contract(
        storage: JsonValue,
        msg: JsonValue,
        arguments: HashMap<String, JsonValue>,
    ) -> Self {
        Self::Contract {
            storage,
            msg,
            arguments,
        }
    }
}
