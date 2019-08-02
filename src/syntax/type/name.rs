//!
//! The syntax type name.
//!

use std::str::FromStr;

#[derive(Debug)]
pub enum Name {
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

impl Name {
    pub fn is_primitive(&self) -> bool {
        match self {
            Name::Field => true,
            Name::Uint8 => true,
            Name::Int8 => true,
            Name::Uint16 => true,
            Name::Int16 => true,
            Name::Uint32 => true,
            Name::Int32 => true,
            Name::Uint64 => true,
            Name::Int64 => true,
            Name::Uint128 => true,
            Name::Int128 => true,
            Name::Bool => true,
            _ => false,
        }
    }

    pub fn is_complex(&self) -> bool {
        match self {
            Name::Struct => true,
            _ => false,
        }
    }

    pub fn is_generic(&self) -> bool {
        match self {
            Name::MemoryVector => true,
            Name::StorageVector => true,
            _ => false,
        }
    }
}

impl FromStr for Name {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "field" => Ok(Name::Field),
            "uint8" => Ok(Name::Uint8),
            "int8" => Ok(Name::Int8),
            "uint16" => Ok(Name::Uint16),
            "int16" => Ok(Name::Int16),
            "uint32" => Ok(Name::Uint32),
            "int32" => Ok(Name::Int32),
            "uint64" => Ok(Name::Uint64),
            "int64" => Ok(Name::Int64),
            "uint128" => Ok(Name::Uint128),
            "int128" => Ok(Name::Int128),
            "bool" => Ok(Name::Bool),
            "struct" => Ok(Name::Struct),
            "memory_vector" => Ok(Name::MemoryVector),
            "storage_vector" => Ok(Name::StorageVector),
            other => Err(other.to_string()),
        }
    }
}
