//!
//! The Zinc tester data.
//!

use std::str::FromStr;

use failure::Fail;
use serde_derive::Deserialize;
use serde_json::Value as JsonValue;

#[derive(Debug, Deserialize, PartialEq)]
pub struct TestCase {
    pub case: String,
    #[serde(default)]
    pub should_panic: bool,
    #[serde(default)]
    pub ignore: bool,
    pub input: JsonValue,
    pub expect: JsonValue,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TestData {
    pub cases: Vec<TestCase>,
    #[serde(default)]
    pub ignore: bool,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "parsing: {}", _0)]
    Parsing(serde_json::Error),
}

static LINE_PREFIX: &str = "//#";

impl FromStr for TestData {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let json = string
            .lines()
            .filter_map(|line| {
                if line.starts_with(LINE_PREFIX) {
                    Some(&line[LINE_PREFIX.len()..])
                } else {
                    None
                }
            })
            .collect::<Vec<&str>>()
            .join("");

        serde_json::from_str(&json).map_err(Error::Parsing)
    }
}
