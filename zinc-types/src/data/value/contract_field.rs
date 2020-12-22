//!
//! The contract value storage field.
//!

use serde::Deserialize;
use serde::Serialize;

use crate::data::r#type::contract_field::ContractField as ContractFieldType;
use crate::data::value::Value;

///
/// The contract value storage field representation.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractField {
    /// The field name.
    pub name: String,
    /// The field value.
    pub value: Value,
    /// Whether the field is public.
    pub is_public: bool,
    /// Whether the field is implicit.
    pub is_implicit: bool,
}

impl ContractField {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(name: String, value: Value, is_public: bool, is_implicit: bool) -> Self {
        Self {
            name,
            value,
            is_public,
            is_implicit,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_from_type(field: ContractFieldType) -> Self {
        Self::new(
            field.name,
            Value::new(field.r#type),
            field.is_public,
            field.is_implicit,
        )
    }
}
