//!
//! The Zinc tester metadata.
//!

pub mod case;
pub mod error;

use std::str::FromStr;

use serde_derive::Deserialize;

use self::case::Case;
use self::error::Error;

///
/// The test file metadata.
///
#[derive(Debug, Deserialize, PartialEq)]
pub struct Metadata {
    /// The test cases.
    pub cases: Vec<Case>,
    /// If the entire test file must be ignored.
    #[serde(default)]
    pub ignore: bool,
}

impl FromStr for Metadata {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let json = string
            .lines()
            .filter(|line| line.starts_with(zinc_const::tester::METADATA_LINE_PREFIX))
            .map(|line| &line[zinc_const::tester::METADATA_LINE_PREFIX.len()..])
            .collect::<Vec<&str>>()
            .join("");

        serde_json::from_str(&json).map_err(Error::Parsing)
    }
}
