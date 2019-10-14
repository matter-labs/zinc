//!
//! Transpiler element descriptor.
//!

use std::fmt;

pub enum Descriptor {
    Index(String),
    Field(String),
}

impl Into<String> for Descriptor {
    fn into(self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Descriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Descriptor::Index(index) => write!(f, "[{}]", index),
            Descriptor::Field(field) => write!(f, ".{}", field),
        }
    }
}
