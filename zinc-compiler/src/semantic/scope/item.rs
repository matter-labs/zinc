//!
//! The semantic analyzer scope item.
//!

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Item {
    Variable,
    Type,
    Variant,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Variable => write!(f, "variable"),
            Self::Type => write!(f, "type"),
            Self::Variant => write!(f, "variant"),
        }
    }
}
