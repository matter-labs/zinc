//!
//! The generator type contract storage field.
//!

use crate::generator::r#type::Type;
use crate::semantic::element::r#type::contract::field::Field as SemanticContractFieldType;

///
/// The contract storage field representation.
///
#[derive(Debug, Clone, PartialEq)]
pub struct ContractField {
    /// The field name.
    pub name: String,
    /// The field type.
    pub r#type: Type,
    /// Whether the field is public.
    pub is_public: bool,
    /// Whether the field is implicit.
    pub is_implicit: bool,
}

impl ContractField {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(name: String, r#type: Type, is_public: bool, is_implicit: bool) -> Self {
        Self {
            name,
            r#type,
            is_public,
            is_implicit,
        }
    }

    ///
    /// Tries to convert the semantic contract field to the IR generator contract field.
    ///
    pub fn try_from_semantic(field: &SemanticContractFieldType) -> Option<Self> {
        Type::try_from_semantic(&field.r#type).map(|r#type| {
            Self::new(
                field.identifier.name.to_owned(),
                r#type,
                field.is_public,
                field.is_implicit,
            )
        })
    }
}

impl Into<zinc_types::ContractFieldType> for ContractField {
    fn into(self) -> zinc_types::ContractFieldType {
        zinc_types::ContractFieldType::new(
            self.name,
            self.r#type.into(),
            self.is_public,
            self.is_implicit,
        )
    }
}
