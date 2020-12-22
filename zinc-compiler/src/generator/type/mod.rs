//!
//! The generator type.
//!

pub mod contract_field;

use num::BigInt;

use crate::semantic::element::r#type::Type as SemanticType;
use crate::semantic::scope::intrinsic::IntrinsicTypeId;

use self::contract_field::ContractField;

///
/// The generator type, which contains only runtime values used by VM.
///
/// Is converted from a semantic type during the bytecode generation.
///
/// Each type has its own logic of writing to the bytecode.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// The IR unit type, which is not written to the bytecode.
    Unit,
    /// The IR boolean type.
    Boolean,
    /// The IR unsigned integer type.
    IntegerUnsigned {
        /// The integer type bitlength.
        bitlength: usize,
    },
    /// The IR signed integer type.
    IntegerSigned {
        /// The integer type bitlength.
        bitlength: usize,
    },
    /// The IR field type.
    Field,
    /// The IR enumeration type.
    Enumeration {
        /// The enumeration type bitlength.
        bitlength: usize,
        /// The enumeration variant list.
        variants: Vec<(String, BigInt)>,
    },
    /// The IR array type.
    Array {
        /// The array element type.
        r#type: Box<Self>,
        /// The array size.
        size: usize,
    },
    /// The IR tuple type.
    Tuple {
        /// The tuple element types.
        types: Vec<Self>,
    },
    /// The IR structure type.
    Structure {
        /// The ordered structure fields array.
        fields: Vec<(String, Self)>,
    },
    /// The IR contract type.
    Contract {
        /// The ordered contract storage fields array.
        fields: Vec<ContractField>,
    },
    /// The IR map type.
    Map {
        /// The map key type.
        key_type: Box<Self>,
        /// The value key type.
        value_type: Box<Self>,
    },
}

impl Type {
    ///
    /// A shortcut constructor.
    ///
    pub fn unit() -> Self {
        Self::Unit
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn boolean() -> Self {
        Self::Boolean
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn integer_unsigned(bitlength: usize) -> Self {
        Self::IntegerUnsigned { bitlength }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn integer_signed(bitlength: usize) -> Self {
        Self::IntegerSigned { bitlength }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn integer(is_signed: bool, bitlength: usize) -> Self {
        if is_signed {
            Self::IntegerSigned { bitlength }
        } else {
            Self::IntegerUnsigned { bitlength }
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn eth_address() -> Self {
        Self::IntegerUnsigned {
            bitlength: zinc_const::size::ETH_ADDRESS,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn field() -> Self {
        Self::Field
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn enumeration(bitlength: usize, variants: Vec<(String, BigInt)>) -> Self {
        Self::Enumeration {
            bitlength,
            variants,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn array(r#type: Self, size: usize) -> Self {
        Self::Array {
            r#type: Box::new(r#type),
            size,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn tuple(types: Vec<Self>) -> Self {
        Self::Tuple { types }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn structure(fields: Vec<(String, Self)>) -> Self {
        Self::Structure { fields }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn contract(fields: Vec<ContractField>) -> Self {
        Self::Contract { fields }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn map(key_type: Self, value_type: Self) -> Self {
        Self::Map {
            key_type: Box::new(key_type),
            value_type: Box::new(value_type),
        }
    }

    ///
    /// The type size in the abstract data stack.
    ///
    pub fn size(&self) -> usize {
        match self {
            Self::Unit => 0,
            Self::Boolean => 1,
            Self::IntegerUnsigned { .. } => 1,
            Self::IntegerSigned { .. } => 1,
            Self::Field => 1,
            Self::Enumeration { .. } => 1,
            Self::Array { r#type, size } => r#type.size() * size,
            Self::Tuple { types } => types.iter().map(|r#type| r#type.size()).sum(),
            Self::Structure { fields } => fields.iter().map(|(_name, r#type)| r#type.size()).sum(),
            Self::Contract { fields } => fields.iter().map(|field| field.r#type.size()).sum(),
            Self::Map { .. } => 0,
        }
    }

    ///
    /// Tries to convert the semantic type to the IR generator type.
    ///
    /// Some types like ranges and functions exist only at compile time and cannot be converted.
    /// In such cases, `None` is returned.
    ///
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
                if inner.type_id == IntrinsicTypeId::StdCollectionsMTreeMap as usize {
                    let key_type = inner
                        .params
                        .as_ref()?
                        .get("K")
                        .map(Self::try_from_semantic)??;
                    let value_type = inner
                        .params
                        .as_ref()?
                        .get("V")
                        .map(Self::try_from_semantic)??;

                    return Some(Self::map(key_type, value_type));
                }

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
            SemanticType::Enumeration(inner) => Some(Self::enumeration(
                inner.bitlength,
                inner
                    .names
                    .to_owned()
                    .into_iter()
                    .zip(inner.values.to_owned())
                    .collect::<Vec<(String, BigInt)>>(),
            )),
            SemanticType::Contract(inner) => {
                match inner
                    .fields
                    .iter()
                    .filter_map(|field| ContractField::try_from_semantic(field))
                    .collect::<Vec<ContractField>>()
                {
                    fields if !fields.is_empty() => Some(Self::contract(fields)),
                    _ => None,
                }
            }
            SemanticType::String(_) => None,
            SemanticType::Range(_) => None,
            SemanticType::RangeInclusive(_) => None,
            SemanticType::Function(_) => None,
        }
    }
}

impl Into<zinc_types::Type> for Type {
    fn into(self) -> zinc_types::Type {
        match self {
            Self::Unit => zinc_types::Type::Unit,
            Self::Boolean => zinc_types::Type::Scalar(zinc_types::ScalarType::Boolean),
            Self::IntegerUnsigned { bitlength } => {
                zinc_types::Type::Scalar(zinc_types::ScalarType::Integer(zinc_types::IntegerType {
                    is_signed: false,
                    bitlength,
                }))
            }
            Self::IntegerSigned { bitlength } => {
                zinc_types::Type::Scalar(zinc_types::ScalarType::Integer(zinc_types::IntegerType {
                    is_signed: true,
                    bitlength,
                }))
            }
            Self::Field => zinc_types::Type::Scalar(zinc_types::ScalarType::Field),
            Self::Enumeration {
                bitlength,
                variants,
            } => zinc_types::Type::Enumeration {
                bitlength,
                variants,
            },
            Self::Array { r#type, size } => {
                let element_type: zinc_types::Type = (*r#type).into();
                zinc_types::Type::Array(Box::new(element_type), size)
            }
            Self::Tuple { types } => {
                zinc_types::Type::Tuple(types.into_iter().map(|r#type| r#type.into()).collect())
            }
            Self::Structure { fields } => zinc_types::Type::Structure(
                fields
                    .into_iter()
                    .map(|(name, r#type)| (name, r#type.into()))
                    .collect(),
            ),
            Self::Contract { fields } => {
                zinc_types::Type::Contract(fields.into_iter().map(|field| field.into()).collect())
            }
            Self::Map {
                key_type,
                value_type,
            } => {
                let key_type: zinc_types::Type = (*key_type).into();
                let value_type: zinc_types::Type = (*value_type).into();
                zinc_types::Type::Map {
                    key_type: Box::new(key_type),
                    value_type: Box::new(value_type),
                }
            }
        }
    }
}

impl Into<Option<zinc_types::ScalarType>> for Type {
    fn into(self) -> Option<zinc_types::ScalarType> {
        match self {
            Self::Boolean => Some(zinc_types::ScalarType::Boolean),
            Self::IntegerUnsigned { bitlength } => {
                Some(zinc_types::ScalarType::Integer(zinc_types::IntegerType {
                    is_signed: false,
                    bitlength,
                }))
            }
            Self::IntegerSigned { bitlength } => {
                Some(zinc_types::ScalarType::Integer(zinc_types::IntegerType {
                    is_signed: true,
                    bitlength,
                }))
            }
            Self::Field => Some(zinc_types::ScalarType::Field),
            _ => None,
        }
    }
}
