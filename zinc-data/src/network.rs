//!
//! The zkSync network type.
//!

use std::fmt;
use std::str::FromStr;

use serde_derive::Deserialize;

///
/// The zkSync network type.
///
#[derive(Debug, Deserialize, Clone, Copy)]
pub enum Network {
    /// The localhost instance.
    #[serde(rename = "localhost")]
    Localhost,
    /// The Rinkeby network.
    #[serde(rename = "rinkeby")]
    Rinkeby,
    /// The Ropsten network.
    #[serde(rename = "ropsten")]
    Ropsten,
}

impl Network {
    ///
    /// The address where the Zandbox server resides.
    ///
    pub fn to_address(&self) -> String {
        // TODO: replace with the real URLs
        match self {
            Self::Localhost => format!("http://localhost:{}", zinc_const::zandbox::PORT),
            Self::Rinkeby => format!("http://localhost:{}", zinc_const::zandbox::PORT),
            Self::Ropsten => format!("http://localhost:{}", zinc_const::zandbox::PORT),
        }
    }
}

impl FromStr for Network {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "localhost" => Ok(Self::Localhost),
            "rinkeby" => Ok(Self::Rinkeby),
            "ropsten" => Ok(Self::Ropsten),
            another => Err(another.to_owned()),
        }
    }
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Localhost => write!(f, "localhost"),
            Self::Rinkeby => write!(f, "rinkeby"),
            Self::Ropsten => write!(f, "ropsten"),
        }
    }
}
