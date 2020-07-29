//!
//! The semantic attribute.
//!

pub mod error;

use std::convert::TryFrom;

use crate::syntax::tree::attribute::Attribute as SyntaxAttribute;

use self::error::Error;

///
/// The semantic attribute.
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Attribute {
    /// The `#[test]` attribute.
    Test,
    /// The `#[should_panic]` attribute.
    ShouldPanic,
    /// The `#[ignore]` attribute.
    Ignore,
}

impl Attribute {
    ///
    /// If the attribute is related to unit tests.
    ///
    pub fn is_test(&self) -> bool {
        match self {
            Self::Test => true,
            Self::ShouldPanic => true,
            Self::Ignore => true,
        }
    }
}

impl TryFrom<SyntaxAttribute> for Attribute {
    type Error = Error;

    fn try_from(value: SyntaxAttribute) -> Result<Self, Self::Error> {
        Ok(match value.identifier.name.as_str() {
            "test" => Self::Test,
            "should_panic" => Self::ShouldPanic,
            "ignore" => Self::Ignore,
            _ => {
                return Err(Error::Unknown {
                    location: value.identifier.location,
                    found: value.identifier.name,
                })
            }
        })
    }
}
