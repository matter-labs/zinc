//!
//! The syntax type keyword.
//!

use std::str::FromStr;

use failure::Fail;
use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub enum Keyword {
    Uint(usize),
    Int(usize),
    Field,
    Bool,
    Struct,
    Enum,
    MemoryVector,
    StorageVector,
}

#[derive(Debug, Fail, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[fail(display = "integer bitlength is not numeric: '{}'", _0)]
    BitlengthNotNumeric(String),
    #[fail(display = "integer bitlength is out of range [1; 253]: {}", _0)]
    BitlengthOutOfRange(usize),
    #[fail(display = "unknown")]
    Unknown,
}

impl FromStr for Keyword {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if let Some("uint") = string.get(..4) {
            let bitlength = &string[4..];
            let bitlength = bitlength
                .parse::<usize>()
                .map_err(|_| Error::BitlengthNotNumeric(bitlength.to_string()))?;
            if !(1..=253).contains(&bitlength) {
                return Err(Error::BitlengthOutOfRange(bitlength));
            }
            return Ok(Keyword::Uint(bitlength));
        }

        if let Some("int") = string.get(..3) {
            let bitlength = &string[3..];
            let bitlength = bitlength
                .parse::<usize>()
                .map_err(|_| Error::BitlengthNotNumeric(bitlength.to_string()))?;
            if !(1..=253).contains(&bitlength) {
                return Err(Error::BitlengthOutOfRange(bitlength));
            }
            return Ok(Keyword::Int(bitlength));
        }

        match string {
            "field" => Ok(Keyword::Field),
            "bool" => Ok(Keyword::Bool),
            "struct" => Ok(Keyword::Struct),
            "enum" => Ok(Keyword::Enum),
            "memory_vector" => Ok(Keyword::MemoryVector),
            "storage_vector" => Ok(Keyword::StorageVector),

            _unknown => Err(Error::Unknown),
        }
    }
}
