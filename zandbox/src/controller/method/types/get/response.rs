//!
//! The contract method types resource GET response.
//!

use serde_derive::Serialize;
use serde_json::Value as JsonValue;

use crate::database::model::method::select::types::Output as MethodSelectTypesOutput;

///
/// The contract method types resource GET response body.
///
#[derive(Debug, Serialize)]
pub struct Body {
    /// The contract method input type.
    pub input_type: JsonValue,
    /// The contract method output type.
    pub output_type: JsonValue,
    /// The contract contract storage structure as JSON.
    pub storage_type: JsonValue,
}

impl From<MethodSelectTypesOutput> for Body {
    fn from(value: MethodSelectTypesOutput) -> Self {
        Self {
            input_type: value.input_type,
            output_type: value.output_type,
            storage_type: value.storage_type,
        }
    }
}
