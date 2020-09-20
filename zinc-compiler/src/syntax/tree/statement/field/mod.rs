//!
//! The contract storage `field` statement.
//!

pub mod builder;

use crate::lexical::token::location::Location;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::r#type::Type;

///
/// The contract storage `field` statement.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    /// The location of the syntax construction.
    pub location: Location,
    /// If the contract storage field is public.
    pub is_public: bool,
    /// If the contract storage field is external.
    pub is_external: bool,
    /// The contract storage field identifier.
    pub identifier: Identifier,
    /// The contract storage field type.
    pub r#type: Type,
}

impl Statement {
    ///
    /// Creates a contract storage `field` statement.
    ///
    pub fn new(
        location: Location,
        is_public: bool,
        is_external: bool,
        identifier: Identifier,
        r#type: Type,
    ) -> Self {
        Self {
            location,
            is_public,
            is_external,
            identifier,
            r#type,
        }
    }
}
