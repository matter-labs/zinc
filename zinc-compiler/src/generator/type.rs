//!
//! The generator type.
//!

use zinc_bytecode::scalar::IntegerType;
use zinc_bytecode::scalar::ScalarType;

use crate::semantic::Type as SemanticType;

#[derive(Debug, Clone)]
pub enum Type {
    Boolean,
    IntegerUnsigned { bitlength: usize },
    IntegerSigned { bitlength: usize },
    Field,
    Array { r#type: Box<Self>, size: usize },
    Group { types: Vec<Self> },
}

impl Type {
    pub fn boolean() -> Self {
        Self::Boolean
    }

    pub fn integer_unsigned(bitlength: usize) -> Self {
        Self::IntegerUnsigned { bitlength }
    }

    pub fn integer_signed(bitlength: usize) -> Self {
        Self::IntegerSigned { bitlength }
    }

    pub fn field() -> Self {
        Self::Field
    }

    pub fn array(r#type: Self, size: usize) -> Self {
        Self::Array {
            r#type: Box::new(r#type),
            size,
        }
    }

    pub fn group(types: Vec<Self>) -> Self {
        Self::Group { types }
    }

    pub fn size(&self) -> usize {
        match self {
            Self::Boolean => 1,
            Self::IntegerUnsigned { .. } => 1,
            Self::IntegerSigned { .. } => 1,
            Self::Field => 1,
            Self::Array { r#type, size } => r#type.size() * size,
            Self::Group { types } => types.iter().map(|r#type| r#type.size()).sum(),
        }
    }

    pub fn try_from_semantic(r#type: &SemanticType) -> Option<Self> {
        match r#type {
            SemanticType::Boolean => Some(Self::boolean()),
            SemanticType::IntegerUnsigned { bitlength } => Some(Self::integer_unsigned(*bitlength)),
            SemanticType::IntegerSigned { bitlength } => Some(Self::integer_signed(*bitlength)),
            SemanticType::Field => Some(Self::field()),
            SemanticType::Array { r#type, size } => {
                Self::try_from_semantic(r#type).map(|r#type| Self::array(r#type, *size))
            }
            SemanticType::Tuple { types } => {
                match types
                    .iter()
                    .filter_map(Self::try_from_semantic)
                    .collect::<Vec<Type>>()
                {
                    types if !types.is_empty() => Some(Self::group(types)),
                    _ => None,
                }
            }
            SemanticType::Structure(structure) => {
                match structure
                    .fields
                    .iter()
                    .filter_map(|(_name, r#type)| Self::try_from_semantic(r#type))
                    .collect::<Vec<Type>>()
                {
                    types if !types.is_empty() => Some(Self::group(types)),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

impl Into<ScalarType> for Type {
    fn into(self) -> ScalarType {
        match self {
            Self::Boolean => ScalarType::Boolean,
            Self::IntegerUnsigned { bitlength } => ScalarType::Integer(IntegerType {
                is_signed: false,
                bitlength,
            }),
            Self::IntegerSigned { bitlength } => ScalarType::Integer(IntegerType {
                is_signed: true,
                bitlength,
            }),
            Self::Field => ScalarType::Field,
            Self::Array { .. } => {
                panic!(crate::generator::PANIC_VALIDATED_DURING_SEMANTIC_ANALYSIS)
            }
            Self::Group { .. } => {
                panic!(crate::generator::PANIC_VALIDATED_DURING_SEMANTIC_ANALYSIS)
            }
        }
    }
}
