//!
//! The Zinc VM bytecode metadata.
//!

use crate::generator::r#type::Type;

///
/// The program method metadata.
///
#[derive(Debug, PartialEq)]
pub struct Method {
    /// The method function type unique ID.
    pub type_id: usize,
    /// The method name.
    pub name: String,
    /// If the method can mutate the contract storage state.
    pub is_mutable: bool,
    /// The entry function input arguments.
    pub input_fields: Vec<(String, bool, Type)>,
    /// The entry function result type.
    pub output_type: Type,
}

impl Method {
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
                .filter_map(|(name, _is_mutable, r#type)| match r#type {
                    Type::Contract { .. } => None,
                    r#type => Some((name.to_owned(), r#type.to_owned())),
                })
                .collect(),
        )
    }
}
