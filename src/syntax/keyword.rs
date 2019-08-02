//!
//! The syntax keyword.
//!

use std::str::FromStr;

#[derive(Debug)]
pub enum Keyword {
    Inputs,
    Witness,
    Require,
    Let,
    Mut,
    For,
    If,
    Else,
    Field,
    Uint8,
    Int8,
    Uint16,
    Int16,
    Uint32,
    Int32,
    Uint64,
    Int64,
    Uint128,
    Int128,
    Bool,
    Struct,
    MemoryVector,
    StorageVector,
}

impl FromStr for Keyword {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "inputs" => Ok(Keyword::Inputs),
            "witness" => Ok(Keyword::Witness),
            "require" => Ok(Keyword::Require),
            "let" => Ok(Keyword::Let),
            "mut" => Ok(Keyword::Mut),
            "for" => Ok(Keyword::For),
            "if" => Ok(Keyword::If),
            "else" => Ok(Keyword::Else),
            "field" => Ok(Keyword::Field),
            "uint8" => Ok(Keyword::Uint8),
            "int8" => Ok(Keyword::Int8),
            "uint16" => Ok(Keyword::Uint16),
            "int16" => Ok(Keyword::Int16),
            "uint32" => Ok(Keyword::Uint32),
            "int32" => Ok(Keyword::Int32),
            "uint64" => Ok(Keyword::Uint64),
            "int64" => Ok(Keyword::Int64),
            "uint128" => Ok(Keyword::Uint128),
            "int128" => Ok(Keyword::Int128),
            "bool" => Ok(Keyword::Bool),
            "struct" => Ok(Keyword::Struct),
            "memory_vector" => Ok(Keyword::MemoryVector),
            "storage_vector" => Ok(Keyword::StorageVector),
            other => Err(other.to_string()),
        }
    }
}
