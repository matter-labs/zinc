//!
//! The interpreter place.
//!

mod error;

pub use self::error::Error;

use crate::interpreter::Value;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Place {
    pub identifier: String,
    pub path: Vec<usize>,
}

impl Place {
    pub fn new(identifier: String) -> Self {
        Self {
            identifier,
            path: Vec::new(),
        }
    }

    pub fn index(&mut self, index: Value) -> Result<(), Error> {
        let index = match index {
            Value::Integer(integer) => integer.to_string().parse::<usize>().expect("Always valid"),
            value => return Err(Error::IndexingExpectedIntegerConstant(value)),
        };

        self.path.push(index);
        Ok(())
    }
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let indexes = self
            .path
            .iter()
            .map(|index| format!("[{}]", index))
            .collect::<Vec<String>>()
            .join("");
        write!(f, "{}{}", self.identifier, indexes)
    }
}
