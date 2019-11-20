//!
//! The outer statement.
//!

use std::fmt;

use crate::syntax::EnumStatement;
use crate::syntax::FnStatement;
use crate::syntax::ModStatement;
use crate::syntax::StructStatement;
use crate::syntax::TypeStatement;
use crate::syntax::UseStatement;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Type(TypeStatement),
    Struct(StructStatement),
    Enum(EnumStatement),
    Fn(FnStatement),
    Mod(ModStatement),
    Use(UseStatement),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Type(statement) => write!(f, "{}", statement),
            Self::Struct(statement) => write!(f, "{}", statement),
            Self::Enum(statement) => write!(f, "{}", statement),
            Self::Fn(statement) => write!(f, "{}", statement),
            Self::Mod(statement) => write!(f, "{}", statement),
            Self::Use(statement) => write!(f, "{}", statement),
        }
    }
}
