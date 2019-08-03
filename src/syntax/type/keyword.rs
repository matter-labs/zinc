//!
//! The syntax type keyword.
//!

use std::str::FromStr;

use failure::Fail;

#[derive(Debug)]
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

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Bitlength is not numeric: {}", _0)]
    BitlengthNotNumeric(std::num::ParseIntError),
    #[fail(display = "Bitlength out of range: {}", _0)]
    BitlengthOutOfRange(usize),
    #[fail(display = "Unknown: {}", _0)]
    Unknown(String),
}

impl FromStr for Keyword {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if let Some("uint") = string.get(..4) {
            let bitlength = (&string[4..])
                .parse::<usize>()
                .map_err(Error::BitlengthNotNumeric)?;
            if !(1..=253).contains(&bitlength) {
                return Err(Error::BitlengthOutOfRange(bitlength));
            }
            return Ok(Keyword::Uint(bitlength));
        }

        if let Some("int") = string.get(..3) {
            let bitlength = (&string[3..])
                .parse::<usize>()
                .map_err(Error::BitlengthNotNumeric)?;
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

            unknown => Err(Error::Unknown(unknown.to_string())),
        }
    }
}
