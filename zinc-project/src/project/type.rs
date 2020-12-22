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
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Type {
    /// The zero-knowledge circuit.
    #[serde(rename = "circuit")]
    Circuit,
    /// The smart-contract.
    #[serde(rename = "contract")]
    Contract,
    /// The library.
    #[serde(rename = "library")]
    Library,
}

impl FromStr for Type {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "circuit" => Ok(Self::Circuit),
            "contract" => Ok(Self::Contract),
            "library" => Ok(Self::Library),
            another => Err(another.to_owned()),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Circuit => write!(f, "circuit"),
            Self::Contract => write!(f, "contract"),
            Self::Library => write!(f, "library"),
        }
    }
}
