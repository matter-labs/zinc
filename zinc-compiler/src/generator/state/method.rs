//!
//! The Zinc VM bytecode metadata.
//!

use crate::generator::r#type::Type;

///
/// The program method metadata.
///
#[derive(Debug, PartialEq)]
pub struct Method {
    /// The method name.
    pub name: String,
    /// If the method can mutate the contract storage state.
    pub is_mutable: bool,
    /// The entry function input arguments.
    pub input_fields: Vec<(String, Type)>,
    /// The entry function result type.
    pub output_type: Type,
}

impl Method {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        name: String,
        is_mutable: bool,
        input_fields: Vec<(String, Type)>,
        output_type: Type,
    ) -> Self {
        Self {
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
                .filter_map(|(name, r#type)| match r#type {
                    Type::Contract { .. } => None,
                    r#type => Some((name.to_owned(), r#type.to_owned())),
                })
                .collect(),
        )
    }
}
