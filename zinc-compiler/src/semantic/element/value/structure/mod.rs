//!
//! The semantic analyzer structure value element.
//!

mod error;

pub use self::error::Error;

use std::fmt;

use crate::semantic::Type;

#[derive(Default, Clone, PartialEq)]
pub struct Structure {
    identifier: String,
    fields: Vec<(String, Type)>,
}

impl Structure {
    pub fn new(identifier: String, fields: Vec<(String, Type)>) -> Self {
        Self { identifier, fields }
    }

    pub fn r#type(&self) -> Type {
        Type::new_structure(self.identifier.to_owned(), self.fields.to_owned())
    }

    pub fn push(&mut self, key: String, r#type: Type) -> Result<(), Error> {
        if self.fields.iter().any(|field| field.0 == key) {
            return Err(Error::FieldAlreadyExists(key));
        }

        self.fields.push((key, r#type));
        Ok(())
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.identifier == other.identifier
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "struct {} {{ {} }}",
            self.identifier,
            self.fields
                .iter()
                .map(|(identifier, r#type)| format!("{}: {}", identifier, r#type))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

impl fmt::Debug for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}
