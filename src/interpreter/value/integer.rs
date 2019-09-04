//!
//! The interpreter integer value.
//!

use std::fmt;

use num_bigint::BigInt;
use serde_derive::Serialize;

use crate::interpreter::OperatorError;
use crate::syntax::TypeVariant;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum Type {
    Uint { bitlength: usize },
    Int { bitlength: usize },
    Field,
}

impl From<TypeVariant> for Type {
    fn from(variant: TypeVariant) -> Self {
        match variant {
            TypeVariant::Uint { bitlength } => Self::Uint { bitlength },
            TypeVariant::Int { bitlength } => Self::Int { bitlength },
            TypeVariant::Field => Self::Field,
            _ => unimplemented!(),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Uint { bitlength } => write!(f, "uint{}", bitlength),
            Self::Int { bitlength } => write!(f, "int{}", bitlength),
            Self::Field => write!(f, "field"),
        }
    }
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Integer {
    #[serde(skip_serializing)]
    pub data: BigInt,
    pub r#type: Type,
}

impl Integer {
    pub fn new(data: BigInt, r#type: Type) -> Self {
        Self { data, r#type }
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.r#type == other.r#type
    }

    pub fn cast(&mut self, to: Type) -> Result<(), OperatorError> {
        match (self.r#type.clone(), to.clone()) {
            (Type::Uint { bitlength: ref b1 }, Type::Uint { bitlength: ref b2 }) if b1 > b2 => {
                return Err(OperatorError::casting_to_lesser_bitlength(
                    self.r#type.clone(),
                    to,
                ));
            }
            (Type::Int { bitlength: ref b1 }, Type::Int { bitlength: ref b2 }) if b1 > b2 => {
                return Err(OperatorError::casting_to_lesser_bitlength(
                    self.r#type.clone(),
                    to,
                ));
            }
            (Type::Uint { bitlength: ref b1 }, Type::Int { bitlength: ref b2 }) if b1 >= b2 => {
                return Err(OperatorError::casting_to_lesser_bitlength(
                    self.r#type.clone(),
                    to,
                ));
            }
            (Type::Int { bitlength: ref b1 }, Type::Uint { bitlength: ref b2 }) => {
                if b1 >= b2 {
                    return Err(OperatorError::casting_to_lesser_bitlength(
                        self.r#type.clone(),
                        to,
                    ));
                }
            }
            (Type::Field, _) => {
                return Err(OperatorError::casting_to_lesser_bitlength(
                    self.r#type.clone(),
                    to,
                ));
            }
            (_, type_2) => self.r#type = type_2,
        }

        Ok(())
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.data, self.r#type)
    }
}
