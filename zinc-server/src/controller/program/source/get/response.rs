//!
//! The program resource GET response.
//!

use serde_derive::Serialize;
use serde_json::Value as JsonValue;

use zinc_postgres::ProgramSelectSourceOutput;

///
/// The program resource GET response body.
///
#[derive(Debug, Serialize)]
pub struct Body {
    /// The program source code.
    pub source: JsonValue,
}

impl From<ProgramSelectSourceOutput> for Body {
    fn from(value: ProgramSelectSourceOutput) -> Self {
        Self {
            source: value.source,
        }
    }
}
