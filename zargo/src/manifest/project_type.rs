//!
//! The Zargo project type.
//!

use std::fmt;
use std::str::FromStr;

use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum ProjectType {
    #[serde(rename = "circuit")]
    Circuit,
    #[serde(rename = "contract")]
    Contract,
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Circuit => write!(f, "circuit"),
            Self::Contract => write!(f, "contract"),
        }
    }
}

impl FromStr for ProjectType {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "circuit" => Ok(Self::Circuit),
            "contract" => Ok(Self::Contract),
            another => Err(another.to_owned()),
        }
    }
}
