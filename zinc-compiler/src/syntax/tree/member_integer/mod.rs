//!
//! The member integer.
//!

mod builder;

pub use self::builder::Builder;

use crate::lexical::IntegerLiteral;
use crate::lexical::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct MemberInteger {
    pub location: Location,
    pub literal: IntegerLiteral,
}

impl MemberInteger {
    pub fn new(location: Location, literal: IntegerLiteral) -> Self {
        Self { location, literal }
    }
}
