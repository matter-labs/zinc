//!
//! The Zinc tester metadata case.
//!

use serde_derive::Deserialize;
use serde_json::Value as JsonValue;

///
/// The test file case.
///
#[derive(Debug, Deserialize, PartialEq)]
pub struct Case {
    /// The case name.
    pub case: String,
    /// The entry which must be run for the case.
    #[serde(default = "crate::default_entry")]
    pub method: String,
    /// The entry input data.
    pub input: JsonValue,
    /// The entry expected output data.
    pub expect: JsonValue,
    /// If the test case must return an error to be successful.
    #[serde(default)]
    pub should_panic: bool,
    /// If the test case must be ignored.
    #[serde(default)]
    pub ignore: bool,
}
