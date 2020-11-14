//!
//! The Zinc project type.
//!

use std::fmt;
use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

///
/// The Zinc project type.
///
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ProjectType {
    /// The zero-knowledge circuit.
    #[serde(rename = "circuit")]
    Circuit,
    /// The smart-contract.
    #[serde(rename = "contract")]
    Contract,
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

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Circuit => write!(f, "circuit"),
            Self::Contract => write!(f, "contract"),
        }
    }
}
