//!
//! The type variant.
//!

use std::collections::BTreeMap;
use std::fmt;

use crate::lexical::IntegerLiteral;
use crate::syntax::Identifier;

#[derive(Debug, Clone, PartialEq)]
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
        size: IntegerLiteral,
    },
    Tuple {
        type_variants: Vec<Self>,
    },
    Structure {
        identifier: Identifier,
        fields: BTreeMap<String, Self>,
    },
    Enumeration {
        identifier: Identifier,
        variants: BTreeMap<String, IntegerLiteral>,
    },
    Function {
        identifier: Identifier,
        arguments: Vec<(String, Self)>,
        return_type: Box<Self>,
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

    pub fn new_array(type_variant: Self, size: IntegerLiteral) -> Self {
        Self::Array {
            type_variant: Box::new(type_variant),
            size,
        }
    }

    pub fn new_tuple(type_variants: Vec<Self>) -> Self {
        Self::Tuple { type_variants }
    }

    pub fn new_structure(identifier: Identifier, fields: Vec<(String, Self)>) -> Self {
        let fields = fields.into_iter().collect::<BTreeMap<String, Self>>();
        Self::Structure { identifier, fields }
    }

    pub fn new_enumeration(
        identifier: Identifier,
        variants: Vec<(String, IntegerLiteral)>,
    ) -> Self {
        let variants = variants
            .into_iter()
            .collect::<BTreeMap<String, IntegerLiteral>>();
        Self::Enumeration {
            identifier,
            variants,
        }
    }

    pub fn new_function(
        identifier: Identifier,
        arguments: Vec<(String, Self)>,
        return_type: Self,
    ) -> Self {
        Self::Function {
            identifier,
            arguments,
            return_type: Box::new(return_type),
        }
    }

    pub fn new_alias(identifier: String) -> Self {
        Self::Alias { identifier }
    }

    pub fn size(&self) -> usize {
        match self {
            Self::Unit => 0,
            Self::Boolean => 1,
            Self::IntegerUnsigned { .. } => 1,
            Self::IntegerSigned { .. } => 1,
            Self::Field => 1,
            _ => 1,
        }
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
            Self::Structure { identifier, fields } => write!(
                f,
                "struct {} {{ {} }}",
                identifier,
                fields
                    .iter()
                    .map(|(identiifer, type_variant)| format!("{}: {}", identiifer, type_variant))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Enumeration {
                identifier,
                variants,
            } => write!(
                f,
                "enum {} {{ {} }}",
                identifier,
                variants
                    .iter()
                    .map(|(identiifer, value)| format!("{} = {}", identiifer, value))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Function {
                identifier,
                arguments,
                return_type,
            } => write!(
                f,
                "fn {}({}) -> {}",
                identifier,
                arguments
                    .iter()
                    .map(|(identiifer, type_variant)| format!("{}: {}", identiifer, type_variant))
                    .collect::<Vec<String>>()
                    .join(", "),
                return_type,
            ),
            Self::Alias { identifier } => write!(f, "{}", identifier),
        }
    }
}
