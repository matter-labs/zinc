//!
//! The module-local statement.
//!

use zinc_lexical::Location;

use crate::tree::statement::contract::Statement as ContractStatement;
use crate::tree::statement::module::Statement as ModStatement;
use crate::tree::statement::r#const::Statement as ConstStatement;
use crate::tree::statement::r#enum::Statement as EnumStatement;
use crate::tree::statement::r#fn::Statement as FnStatement;
use crate::tree::statement::r#impl::Statement as ImplStatement;
use crate::tree::statement::r#struct::Statement as StructStatement;
use crate::tree::statement::r#type::Statement as TypeStatement;
use crate::tree::statement::r#use::Statement as UseStatement;

///
/// The module-level statement.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// The `const` statement.
    Const(ConstStatement),
    /// The `type` statement.
    Type(TypeStatement),
    /// The `struct` statement.
    Struct(StructStatement),
    /// The `enum` statement.
    Enum(EnumStatement),
    /// The `fn` statement.
    Fn(FnStatement),
    /// The `mod` statement.
    Mod(ModStatement),
    /// The `use` statement.
    Use(UseStatement),
    /// The `impl` statement.
    Impl(ImplStatement),
    /// The `contract` statement.
    Contract(ContractStatement),
    /// The empty `;` statement.
    Empty(Location),
}

impl Statement {
    ///
    /// The statement location.
    ///
    pub fn location(&self) -> Location {
        match self {
            Self::Const(inner) => inner.location,
            Self::Type(inner) => inner.location,
            Self::Struct(inner) => inner.location,
            Self::Enum(inner) => inner.location,
            Self::Fn(inner) => inner.location,
            Self::Mod(inner) => inner.location,
            Self::Use(inner) => inner.location,
            Self::Impl(inner) => inner.location,
            Self::Contract(inner) => inner.location,
            Self::Empty(location) => *location,
        }
    }
}
