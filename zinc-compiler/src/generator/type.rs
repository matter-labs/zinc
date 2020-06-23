//!
//! The generator type.
//!

use zinc_bytecode::DataType;
use zinc_bytecode::IntegerType;
use zinc_bytecode::ScalarType;

use crate::semantic::element::r#type::Type as SemanticType;

///
/// The generator type, which contains only runtime values used by VM.
///
/// Is converted from a semantic type during the bytecode generation.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Unit,
    Boolean,
    IntegerUnsigned { bitlength: usize },
    IntegerSigned { bitlength: usize },
    Field,
    Array { r#type: Box<Self>, size: usize },
    Tuple { types: Vec<Self> },
    Structure { fields: Vec<(String, Self)> },
    Contract { fields: Vec<(String, Self)> },
}

impl Type {
    pub fn unit() -> Self {
        Self::Unit
    }

    pub fn boolean() -> Self {
        Self::Boolean
    }

    pub fn integer_unsigned(bitlength: usize) -> Self {
        Self::IntegerUnsigned { bitlength }
    }

    pub fn integer_signed(bitlength: usize) -> Self {
        Self::IntegerSigned { bitlength }
    }

    pub fn integer(is_signed: bool, bitlength: usize) -> Self {
        if is_signed {
            Self::IntegerSigned { bitlength }
        } else {
            Self::IntegerUnsigned { bitlength }
        }
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

    pub fn tuple(types: Vec<Self>) -> Self {
        Self::Tuple { types }
    }

    pub fn structure(fields: Vec<(String, Self)>) -> Self {
        Self::Structure { fields }
    }

    pub fn contract(fields: Vec<(String, Self)>) -> Self {
        Self::Contract { fields }
    }

    pub fn size(&self) -> usize {
        match self {
            Self::Unit => 0,
            Self::Boolean => 1,
            Self::IntegerUnsigned { .. } => 1,
            Self::IntegerSigned { .. } => 1,
            Self::Field => 1,
            Self::Array { r#type, size } => r#type.size() * size,
            Self::Tuple { types } => types.iter().map(|r#type| r#type.size()).sum(),
            Self::Structure { fields } => fields.iter().map(|(_name, r#type)| r#type.size()).sum(),
            Self::Contract { fields } => fields.iter().map(|(_name, r#type)| r#type.size()).sum(),
        }
    }

    pub fn try_from_semantic(r#type: &SemanticType) -> Option<Self> {
        match r#type {
            SemanticType::Unit(_) => Some(Self::unit()),
            SemanticType::Boolean(_) => Some(Self::boolean()),
            SemanticType::IntegerUnsigned { bitlength, .. } => {
                Some(Self::integer_unsigned(*bitlength))
            }
            SemanticType::IntegerSigned { bitlength, .. } => Some(Self::integer_signed(*bitlength)),
            SemanticType::Field(_) => Some(Self::field()),
            SemanticType::Array(inner) => Self::try_from_semantic(&*inner.r#type)
                .map(|r#type| Self::array(r#type, inner.size)),
            SemanticType::Tuple(inner) => {
                match inner
                    .types
                    .iter()
                    .filter_map(Self::try_from_semantic)
                    .collect::<Vec<Type>>()
                {
                    types if !types.is_empty() => Some(Self::tuple(types)),
                    _ => None,
                }
            }
            SemanticType::Structure(inner) => {
                match inner
                    .fields
                    .iter()
                    .filter_map(|(name, r#type)| {
                        Self::try_from_semantic(r#type).map(|r#type| (name.to_owned(), r#type))
                    })
                    .collect::<Vec<(String, Type)>>()
                {
                    fields if !fields.is_empty() => Some(Self::structure(fields)),
                    _ => None,
                }
            }
            SemanticType::Enumeration(inner) => Some(Self::integer_unsigned(inner.bitlength)),
            SemanticType::Contract(inner) => {
                match inner
                    .fields
                    .iter()
                    .filter_map(|(name, r#type)| {
                        Self::try_from_semantic(r#type).map(|r#type| (name.to_owned(), r#type))
                    })
                    .collect::<Vec<(String, Type)>>()
                {
                    fields if !fields.is_empty() => Some(Self::contract(fields)),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

impl Into<DataType> for Type {
    fn into(self) -> DataType {
        match self {
            Self::Unit => DataType::Unit,
            Self::Boolean => DataType::Scalar(ScalarType::Boolean),
            Self::IntegerUnsigned { bitlength } => {
                DataType::Scalar(ScalarType::Integer(IntegerType {
                    is_signed: false,
                    bitlength,
                }))
            }
            Self::IntegerSigned { bitlength } => {
                DataType::Scalar(ScalarType::Integer(IntegerType {
                    is_signed: true,
                    bitlength,
                }))
            }
            Self::Field => DataType::Scalar(ScalarType::Field),
            Self::Array { r#type, size } => {
                let element_type: DataType = (*r#type).into();
                DataType::Array(Box::new(element_type), size)
            }
            Self::Tuple { types } => {
                DataType::Tuple(types.into_iter().map(|r#type| r#type.into()).collect())
            }
            Self::Structure { fields } => DataType::Structure(
                fields
                    .into_iter()
                    .map(|(name, r#type)| (name, r#type.into()))
                    .collect(),
            ),
            Self::Contract { fields } => DataType::Contract(
                fields
                    .into_iter()
                    .map(|(name, r#type)| (name, r#type.into()))
                    .collect(),
            ),
        }
    }
}

impl Into<Option<ScalarType>> for Type {
    fn into(self) -> Option<ScalarType> {
        match self {
            Self::Boolean => Some(ScalarType::Boolean),
            Self::IntegerUnsigned { bitlength } => Some(ScalarType::Integer(IntegerType {
                is_signed: false,
                bitlength,
            })),
            Self::IntegerSigned { bitlength } => Some(ScalarType::Integer(IntegerType {
                is_signed: true,
                bitlength,
            })),
            Self::Field => Some(ScalarType::Field),
            _ => None,
        }
    }
}
