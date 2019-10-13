//!
//! Transpiler permanent element.
//!

use std::fmt;

use crate::Descriptor;

pub struct Element {
    pub identifier: String,
    pub descriptors: Vec<Descriptor>,
}

impl Element {
    pub fn new(identifier: String) -> Self {
        Self {
            identifier,
            descriptors: Default::default(),
        }
    }

    pub fn push_descriptor(&mut self, descriptor: Descriptor) {
        self.descriptors.push(descriptor);
    }
}

impl Into<String> for Element {
    fn into(self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.identifier,
            self.descriptors
                .iter()
                .map(|descriptor| match descriptor {
                    Descriptor::Index(index) => format!("[{}]", index),
                    Descriptor::Field(field) => format!(".{}", field),
                })
                .collect::<Vec<String>>()
                .join(""),
        )
    }
}
