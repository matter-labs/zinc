//!
//! The Zinc VM bytecode metadata.
//!

use crate::generator::r#type::Type;

///
/// The application entry metadata.
///
#[derive(Debug, PartialEq)]
pub struct Metadata {
    /// The entry name.
    pub entry_name: String,
    /// The entry function input arguments.
    pub input_fields: Vec<(String, Type)>,
    /// The entry function result type.
    pub output_type: Type,
}

impl Metadata {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(entry_name: String, input_fields: Vec<(String, Type)>, output_type: Type) -> Self {
        Self {
            entry_name,
            input_fields,
            output_type,
        }
    }

    ///
    /// The entry function input arguments size.
    ///
    pub fn input_size(&self) -> usize {
        self.input_fields
            .iter()
            .map(|(_name, r#type)| match r#type {
                Type::Contract { .. } => 0,
                r#type => r#type.size(),
            })
            .sum()
    }

    ///
    /// The entry function output type size.
    ///
    pub fn output_size(&self) -> usize {
        self.output_type.size()
    }

    ///
    /// Wraps the input arguments into a structure, e.g. for JSON output.
    ///
    pub fn input_fields_as_struct(&self) -> Type {
        Type::structure(
            self.input_fields
                .iter()
                .filter_map(|(name, r#type)| match r#type {
                    Type::Contract { .. } => None,
                    r#type => Some((name.to_owned(), r#type.to_owned())),
                })
                .collect(),
        )
    }
}
