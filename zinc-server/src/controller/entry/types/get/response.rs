//!
//! The program entry types resource GET response.
//!

use serde_derive::Serialize;
use serde_json::Value as JsonValue;

use zinc_postgres::EntrySelectTemplatesOutput;

///
/// The program entry types resource GET response body.
///
#[derive(Debug, Serialize)]
pub struct Body {
    /// The program entry input type.
    pub input_type: JsonValue,
    /// The program entry output type.
    pub output_type: JsonValue,
    /// The program contract storage structure as JSON.
    pub storage_type: JsonValue,
}

impl From<EntrySelectTemplatesOutput> for Body {
    fn from(value: EntrySelectTemplatesOutput) -> Self {
        Self {
            input_type: value.input_type,
            output_type: value.output_type,
            storage_type: value.storage_type,
        }
    }
}
