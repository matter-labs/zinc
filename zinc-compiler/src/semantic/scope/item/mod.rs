//!
//! The semantic analyzer scope item.
//!

pub mod variant;

use std::fmt;

use crate::lexical::token::location::Location;

use self::variant::Variant;

///
/// An item declared within a scope.
///
/// Items are variables, constants, types, modules, etc.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub variant: Variant,
    pub location: Option<Location>,
}

impl Item {
    pub fn new(variant: Variant, location: Option<Location>) -> Self {
        Self { variant, location }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.location {
            Some(location) => write!(f, "{}, declared at {}", self.variant, location),
            None => write!(f, "{}", self.variant),
        }
    }
}
