//!
//! The type variant.
//!

use std::collections::BTreeMap;
use std::fmt;

use serde_derive::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "name")]
pub enum Variant {
    Unit,
    Boolean,
    IntegerUnsigned {
        bitlength: usize,
    },
    IntegerSigned {
        bitlength: usize,
    },
    Field,
    Array {
        type_variant: Box<Self>,
        size: usize,
    },
    Tuple {
        type_variants: Vec<Self>,
    },
    Structure {
        fields: BTreeMap<String, Self>,
    },
    Enumeration {
        variants: BTreeMap<String, usize>,
    },
    Alias {
        identifier: String,
    },
}

impl Variant {
    pub fn new_unit() -> Self {
        Self::Unit
    }

    pub fn new_boolean() -> Self {
        Self::Boolean
    }

    pub fn new_integer(is_signed: bool, bitlength: usize) -> Self {
        if is_signed {
            Self::new_integer_signed(bitlength)
        } else {
            Self::new_integer_unsigned(bitlength)
        }
    }

    pub fn new_integer_unsigned(bitlength: usize) -> Self {
        Self::IntegerUnsigned { bitlength }
    }

    pub fn new_integer_signed(bitlength: usize) -> Self {
        Self::IntegerSigned { bitlength }
    }

    pub fn new_field() -> Self {
        Self::Field
    }

    pub fn new_array(type_variant: Self, size: usize) -> Self {
        Self::Array {
            type_variant: Box::new(type_variant),
            size,
        }
    }

    pub fn new_tuple(type_variants: Vec<Self>) -> Self {
        Self::Tuple { type_variants }
    }

    pub fn new_structure(fields: Vec<(String, Self)>) -> Self {
        let fields = fields.into_iter().collect::<BTreeMap<String, Self>>();
        Self::Structure { fields }
    }

    pub fn new_enumeration(variants: Vec<(String, usize)>) -> Self {
        let variants = variants.into_iter().collect::<BTreeMap<String, usize>>();
        Self::Enumeration { variants }
    }

    pub fn new_alias(identifier: String) -> Self {
        Self::Alias { identifier }
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unit => write!(f, "()"),
            Self::Boolean => write!(f, "bool"),
            Self::IntegerUnsigned { bitlength } => write!(f, "u{}", bitlength),
            Self::IntegerSigned { bitlength } => write!(f, "i{}", bitlength),
            Self::Field => write!(f, "field"),
            Self::Array { type_variant, size } => write!(f, "[{}; {}]", type_variant, size),
            Self::Tuple { type_variants } => write!(
                f,
                "({})",
                type_variants
                    .iter()
                    .map(|type_variant| type_variant.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Structure { fields } => write!(
                f,
                "{{ {} }}",
                fields
                    .iter()
                    .map(|(identiifer, type_variant)| format!("{}: {}", identiifer, type_variant))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Enumeration { variants } => write!(
                f,
                "{{ {} }}",
                variants
                    .iter()
                    .map(|(identiifer, value)| format!("{} = {}", identiifer, value))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Alias { identifier } => write!(f, "{}", identifier),
        }
    }
}
