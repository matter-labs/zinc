//!
//! The template resource GET response.
//!

use serde_derive::Serialize;

use crate::database::model::template::select::single::Output as TemplateSelectOutput;

///
/// The template resource GET response body.
///
#[derive(Debug, Serialize)]
pub struct Body {
    /// The template bytecode.
    pub bytecode: Vec<u8>,
    /// The template verifying key as a byte array.
    pub verifying_key: Vec<u8>,
}

impl From<TemplateSelectOutput> for Body {
    fn from(value: TemplateSelectOutput) -> Self {
        Self {
            bytecode: value.bytecode,
            verifying_key: value.verifying_key,
        }
    }
}
