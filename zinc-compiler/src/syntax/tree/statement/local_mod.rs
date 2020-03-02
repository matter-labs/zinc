//!
//! The module-local statement.
//!

use crate::lexical::Location;
use crate::syntax::tree::statement::module::Statement as ModStatement;
use crate::syntax::tree::statement::r#const::Statement as ConstStatement;
use crate::syntax::tree::statement::r#enum::Statement as EnumStatement;
use crate::syntax::tree::statement::r#fn::Statement as FnStatement;
use crate::syntax::tree::statement::r#impl::Statement as ImplStatement;
use crate::syntax::tree::statement::r#static::Statement as StaticStatement;
use crate::syntax::tree::statement::r#struct::Statement as StructStatement;
use crate::syntax::tree::statement::r#type::Statement as TypeStatement;
use crate::syntax::tree::statement::r#use::Statement as UseStatement;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Const(ConstStatement),
    Static(StaticStatement),
    Type(TypeStatement),
    Struct(StructStatement),
    Enum(EnumStatement),
    Fn(FnStatement),
    Mod(ModStatement),
    Use(UseStatement),
    Impl(ImplStatement),
    Empty(Location),
}

impl Statement {
    pub fn location(&self) -> Location {
        match self {
            Self::Const(inner) => inner.location,
            Self::Static(inner) => inner.location,
            Self::Type(inner) => inner.location,
            Self::Struct(inner) => inner.location,
            Self::Enum(inner) => inner.location,
            Self::Fn(inner) => inner.location,
            Self::Mod(inner) => inner.location,
            Self::Use(inner) => inner.location,
            Self::Impl(inner) => inner.location,
            Self::Empty(location) => *location,
        }
    }
}
