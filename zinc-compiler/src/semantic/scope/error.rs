//!
//! The semantic analyzer scope error.
//!

use crate::lexical::token::location::Location;

///
/// The semantic analyzer scope error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The item is undeclared within the current scope stack.
    ItemUndeclared {
        /// The error location data.
        location: Location,
        /// The undeclared item name.
        name: String,
    },
    /// The item is already declared within the current scope stack.
    ItemRedeclared {
        /// The error location data.
        location: Location,
        /// The redeclared item name.
        name: String,
        /// The location where the item is declared for the first item. `None` for built-in items.
        reference: Option<Location>,
    },
    /// The item is not a namespace, and cannot be a part of a path expression.
    ItemIsNotANamespace {
        /// The error location data.
        location: Location,
        /// The non-namespace item name.
        name: String,
    },
    /// The associated item is accessed without specifying its namespace.
    AssociatedItemWithoutOwner {
        /// The error location data.
        location: Location,
        /// The associated item name.
        name: String,
    },
    /// Another contract is already declared within the scope stack.
    /// Only one contract is allowed per application.
    ContractRedeclared {
        /// The error location data.
        location: Location,
        /// The location where the first contract is declared.
        reference: Location,
    },
    /// There is a reference loop between items. That is, there are some items referencing each
    /// other. Perhaps, not directly, that is, through one or more other items.
    ReferenceLoop {
        /// The error location data.
        location: Location,
    },
}
