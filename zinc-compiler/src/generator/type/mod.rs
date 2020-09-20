//!
//! The generator type.
//!

pub mod contract_field;

use num_bigint::BigInt;

use zinc_build::IntegerType;
use zinc_build::ScalarType;
use zinc_build::Type as BuildType;

use crate::semantic::element::r#type::Type as SemanticType;

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
    /// The type size in the Zinc VM data stack.
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

impl Into<BuildType> for Type {
    fn into(self) -> BuildType {
        match self {
            Self::Unit => BuildType::Unit,
            Self::Boolean => BuildType::Scalar(ScalarType::Boolean),
            Self::IntegerUnsigned { bitlength } => {
                BuildType::Scalar(ScalarType::Integer(IntegerType {
                    is_signed: false,
                    bitlength,
                }))
            }
            Self::IntegerSigned { bitlength } => {
                BuildType::Scalar(ScalarType::Integer(IntegerType {
                    is_signed: true,
                    bitlength,
                }))
            }
            Self::Field => BuildType::Scalar(ScalarType::Field),
            Self::Enumeration {
                bitlength,
                variants,
            } => BuildType::Enumeration {
                bitlength,
                variants,
            },
            Self::Array { r#type, size } => {
                let element_type: BuildType = (*r#type).into();
                BuildType::Array(Box::new(element_type), size)
            }
            Self::Tuple { types } => {
                BuildType::Tuple(types.into_iter().map(|r#type| r#type.into()).collect())
            }
            Self::Structure { fields } => BuildType::Structure(
                fields
                    .into_iter()
                    .map(|(name, r#type)| (name, r#type.into()))
                    .collect(),
            ),
            Self::Contract { fields } => {
                BuildType::Contract(fields.into_iter().map(|field| field.into()).collect())
            }
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
