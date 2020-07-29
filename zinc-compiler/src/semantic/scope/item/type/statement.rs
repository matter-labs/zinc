//!
//! The semantic analyzer scope type item statement.
//!

use crate::lexical::token::location::Location;
use crate::semantic::analyzer::statement::r#fn::Context as FnStatementAnalyzerContext;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::statement::contract::Statement as ContractStatement;
use crate::syntax::tree::statement::r#enum::Statement as EnumStatement;
use crate::syntax::tree::statement::r#fn::Statement as FnStatement;
use crate::syntax::tree::statement::r#struct::Statement as StructStatement;
use crate::syntax::tree::statement::r#type::Statement as TypeStatement;

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
    Fn(FnStatement, FnStatementAnalyzerContext),
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
            Self::Fn(inner, _context) => inner.location,
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
            Self::Fn(inner, _context) => &inner.identifier,
            Self::Contract(inner) => &inner.identifier,
        }
    }
}
