//!
//! The semantic analyzer place element descriptor.
//!

use std::fmt;

use crate::semantic::IntegerConstant;
use crate::semantic::IntegerValue;

#[derive(Debug, Clone, PartialEq)]
pub enum Descriptor {
    ArrayIndexConstant(IntegerConstant),
    ArrayIndexValue(IntegerValue),
    TupleField(usize),
    StructureField(String),
}

impl fmt::Display for Descriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ArrayIndexConstant(constant) => write!(f, "[{}]", constant),
            Self::ArrayIndexValue(value) => write!(f, "[{}]", value),
            Self::TupleField(field) => write!(f, ".{}", field),
            Self::StructureField(field) => write!(f, ".{}", field),
        }
    }
}
