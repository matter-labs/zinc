//!
//! The semantic analyzer place element.
//!

mod descriptor;
mod error;

pub use self::descriptor::Descriptor;
pub use self::error::Error;

use std::fmt;

use crate::lexical::Location;
use crate::semantic::IntegerConstant;

#[derive(Debug, Clone, PartialEq)]
pub struct Place {
    pub location: Location,
    pub identifier: String,
    pub descriptors: Vec<Descriptor>,
}

impl Place {
    pub fn new(location: Location, identifier: String) -> Self {
        Self {
            location,
            identifier,
            descriptors: Vec::new(),
        }
    }

    pub fn name(&self) -> String {
        self.identifier.clone()
    }

    pub fn index(&mut self, constant: &IntegerConstant) -> Result<(), Error> {
        let index = constant.to_usize().map_err(Error::IndexConstant)?;
        self.descriptors.push(Descriptor::ArrayIndex(index));
        Ok(())
    }

    pub fn access_tuple(&mut self, field: usize) {
        self.descriptors.push(Descriptor::TupleField(field));
    }

    pub fn access_structure(&mut self, identifier: &str) {
        self.descriptors
            .push(Descriptor::StructureField(identifier.to_owned()));
    }
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.identifier,
            self.descriptors
                .iter()
                .map(|descriptor| descriptor.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}
