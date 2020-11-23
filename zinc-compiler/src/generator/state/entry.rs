//!
//! The Zinc VM bytecode metadata.
//!

use crate::generator::r#type::Type;

///
/// The application entry metadata.
///
#[derive(Debug, PartialEq)]
pub struct Entry {
    /// The entry function type unique ID.
    pub type_id: usize,
    /// The entry name.
    pub name: String,
    /// If the entry can mutate the contract storage state. Only for contracts.
    pub is_mutable: bool,
    /// The entry function input arguments.
    pub input_fields: Vec<(String, bool, Type)>,
    /// The entry function result type.
    pub output_type: Type,
}

impl Entry {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        type_id: usize,
        name: String,
        is_mutable: bool,
        input_fields: Vec<(String, bool, Type)>,
        output_type: Type,
    ) -> Self {
        Self {
            type_id,
            name,
            is_mutable,
            input_fields,
            output_type,
        }
    }

    ///
    /// Wraps the input arguments into a structure, e.g. for JSON output.
    ///
    pub fn input_fields_as_struct(&self) -> Type {
        Type::structure(
            self.input_fields
                .iter()
                .map(|(name, _is_mutable, r#type)| (name.to_owned(), r#type.to_owned()))
                .collect(),
        )
    }
}
