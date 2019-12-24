//!
//! The semantic analyzer place element.
//!

mod descriptor;
mod error;
mod resolution_time;

pub use self::descriptor::Descriptor;
pub use self::error::Error;
pub use self::resolution_time::ResolutionTime;

use std::fmt;

use crate::lexical::Location;
use crate::semantic::IntegerConstant;
use crate::semantic::IntegerValue;
use crate::syntax::MemberString;

#[derive(Debug, Clone, PartialEq)]
pub struct Place {
    pub location: Location,
    pub path: Vec<MemberString>,
    pub descriptors: Vec<Descriptor>,
    pub resolution_time: ResolutionTime,
}

impl Place {
    pub fn new(location: Location, first: MemberString) -> Self {
        Self {
            location,
            path: vec![first],
            descriptors: Vec::new(),
            resolution_time: ResolutionTime::Static,
        }
    }

    pub fn path(&mut self, member_string: &MemberString) -> Result<(), Error> {
        if !self.descriptors.is_empty() {
            return Err(Error::PathDescripted(self.to_string()));
        }
        self.path.push(member_string.to_owned());
        Ok(())
    }

    pub fn index_constant(&mut self, constant: &IntegerConstant) {
        self.descriptors
            .push(Descriptor::ArrayIndexConstant(constant.to_owned()));
    }

    pub fn index_value(&mut self, value: &IntegerValue) {
        self.descriptors
            .push(Descriptor::ArrayIndexValue(value.to_owned()));
        self.resolution_time = ResolutionTime::Dynamic;
    }

    pub fn access_tuple(&mut self, field: usize) {
        self.descriptors.push(Descriptor::TupleField(field));
    }

    pub fn access_structure(&mut self, member_string: &MemberString) {
        self.descriptors
            .push(Descriptor::StructureField(member_string.name.to_owned()));
    }
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.path
                .iter()
                .map(|identifier| identifier.name.to_owned())
                .collect::<Vec<String>>()
                .join("::"),
            self.descriptors
                .iter()
                .map(|descriptor| descriptor.to_string())
                .collect::<Vec<String>>()
                .join(""),
        )
    }
}
