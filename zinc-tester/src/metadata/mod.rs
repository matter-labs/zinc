//!
//! The Zinc tester metadata.
//!

pub mod error;

use std::str::FromStr;

use serde_derive::Deserialize;
use serde_json::Value as JsonValue;

use self::error::Error;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Case {
    pub case: String,
    #[serde(default = "crate::default_entry")]
    pub entry: String,
    pub input: JsonValue,
    pub expect: JsonValue,
    #[serde(default)]
    pub should_panic: bool,
    #[serde(default)]
    pub ignore: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Metadata {
    pub cases: Vec<Case>,
    #[serde(default)]
    pub ignore: bool,
}

impl FromStr for Metadata {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let json = string
            .lines()
            .filter(|line| line.starts_with(crate::TEST_LINE_PREFIX))
            .map(|line| &line[crate::TEST_LINE_PREFIX.len()..])
            .collect::<Vec<&str>>()
            .join("");

        serde_json::from_str(&json).map_err(Error::Parsing)
    }
}
