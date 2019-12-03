//!
//! The semantic analyzer place element.
//!

mod descriptor;
mod error;

pub use self::descriptor::Descriptor;
pub use self::error::Error;

use std::fmt;

use crate::semantic::Constant;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Place {
    pub identifier: String,
    pub elements: Vec<Descriptor>,
}

impl Place {
    pub fn new(identifier: String) -> Self {
        Self {
            identifier,
            elements: Vec::new(),
        }
    }

    pub fn name(&self) -> String {
        self.identifier.clone()
    }

    pub fn index(&mut self, constant: Constant) -> Result<(), Error> {
        let index = constant.to_usize().map_err(Error::IndexConstant)?;
        self.elements.push(Descriptor::ArrayIndex(index));
        Ok(())
    }

    pub fn access_tuple(&mut self, constant: Constant) -> Result<(), Error> {
        let field = constant.to_usize().map_err(Error::TupleAccessConstant)?;
        self.elements.push(Descriptor::TupleField(field));
        Ok(())
    }

    pub fn access_structure(&mut self, identifier: String) {
        self.elements.push(Descriptor::StructureField(identifier));
    }
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.identifier,
            self.elements
                .iter()
                .map(|element| match element {
                    Descriptor::ArrayIndex(index) => format!("[{}]", index),
                    Descriptor::TupleField(index) => format!(".{}", index),
                    Descriptor::StructureField(identifier) => format!(".{}", identifier),
                })
                .collect::<Vec<String>>()
                .join("")
        )
    }
}
