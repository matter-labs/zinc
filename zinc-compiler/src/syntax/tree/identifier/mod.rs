//!
//! The identifier.
//!

pub mod builder;

use crate::lexical::token::lexeme::keyword::Keyword;
use crate::lexical::token::location::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub location: Location,
    pub name: String,
}

impl Identifier {
    pub fn new(location: Location, name: String) -> Self {
        Self { location, name }
    }

    pub fn is_self(&self) -> bool {
        self.name == Keyword::SelfLowercase.to_string()
    }
}
