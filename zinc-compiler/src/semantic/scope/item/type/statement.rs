//!
//! The semantic analyzer scope type item statement.
//!

use zinc_lexical::Location;
use zinc_syntax::ContractStatement;
use zinc_syntax::EnumStatement;
use zinc_syntax::FnStatement;
use zinc_syntax::Identifier;
use zinc_syntax::StructStatement;
use zinc_syntax::TypeStatement;

///
/// The item declaration statement, which may be resolved
///
#[derive(Debug, Clone)]
pub enum Statement {
    /// The `type` statement.
    Type(TypeStatement),
    /// The `struct` statement.
    Struct(StructStatement),
    /// The `enum` statement.
    Enum(EnumStatement),
    /// The `fn` statement.
    Fn(FnStatement),
    /// The `contract` statement.
    Contract(ContractStatement),
}

impl Statement {
    ///
    /// The location where the statement is declared.
    ///
    pub fn location(&self) -> Location {
        match self {
            Self::Type(inner) => inner.location,
            Self::Struct(inner) => inner.location,
            Self::Enum(inner) => inner.location,
            Self::Fn(inner) => inner.location,
            Self::Contract(inner) => inner.location,
        }
    }

    ///
    /// The identifier of the item, declared with the statement.
    ///
    pub fn identifier(&self) -> &Identifier {
        match self {
            Self::Type(inner) => &inner.identifier,
            Self::Struct(inner) => &inner.identifier,
            Self::Enum(inner) => &inner.identifier,
            Self::Fn(inner) => &inner.identifier,
            Self::Contract(inner) => &inner.identifier,
        }
    }
}
