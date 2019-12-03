//!
//! The outer statement.
//!

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
