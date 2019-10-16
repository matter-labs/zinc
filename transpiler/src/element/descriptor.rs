//!
//! Transpiler element descriptor.
//!

use std::fmt;

#[derive(Debug)]
pub enum Descriptor {
    Array(usize),
    Tuple(usize),
    Structure(String),
}

impl Into<String> for Descriptor {
    fn into(self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Descriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Descriptor::Array(index) => write!(f, "[{}]", index),
            Descriptor::Tuple(field) => write!(f, ".{}", field),
            Descriptor::Structure(field) => write!(f, ".{}", field),
        }
    }
}
