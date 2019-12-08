//!
//! The semantic analyzer place element descriptor.
//!

use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Descriptor {
    ArrayIndex(usize),
    TupleField(usize),
    StructureField(String),
}

impl Descriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ArrayIndex(index) => write!(f, "[{}]", index),
            Self::TupleField(field) => write!(f, ".{}", field),
            Self::StructureField(field) => write!(f, ".{}", field),
        }
    }
}

impl fmt::Display for Descriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

impl fmt::Debug for Descriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}
