//!
//! The Zinc VM bytecode metadata.
//!

use crate::generator::r#type::Type;

#[derive(Debug, PartialEq)]
pub struct Metadata {
    pub entry_name: String,
    pub input_fields: Vec<(String, Type)>,
    pub output_type: Type,
}

impl Metadata {
    pub fn new(entry_name: String, input_fields: Vec<(String, Type)>, output_type: Type) -> Self {
        Self {
            entry_name,
            input_fields,
            output_type,
        }
    }

    pub fn input_size(&self) -> usize {
        self.input_fields
            .iter()
            .map(|(_name, r#type)| r#type.size())
            .sum()
    }

    pub fn output_size(&self) -> usize {
        self.output_type.size()
    }

    pub fn input_fields_as_struct(&self) -> Type {
        Type::structure(
            self.input_fields
                .iter()
                .map(|(name, r#type)| (name.to_owned(), r#type.to_owned()))
                .collect(),
        )
    }
}
