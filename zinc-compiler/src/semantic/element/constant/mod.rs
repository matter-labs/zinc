//!
//! The semantic analyzer constant element.
//!

pub mod error;
pub mod integer;
pub mod range;
pub mod range_inclusive;

use std::fmt;

use num_bigint::BigInt;
use num_traits::One;
use num_traits::Zero;

use zinc_bytecode::Instruction;

use crate::semantic::caster::Caster;
use crate::semantic::element::r#type::Type;
use crate::syntax::BooleanLiteral;

use self::error::Error;
use self::integer::Integer;
use self::range::Range;
use self::range_inclusive::RangeInclusive;
use zinc_bytecode::scalar::ScalarType;

#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    Unit,
    Boolean(bool),
    Integer(Integer),
    Range(Range),
    RangeInclusive(RangeInclusive),
    String(String),
}

impl Constant {
    pub fn r#type(&self) -> Type {
        match self {
            Self::Unit => Type::unit(),
            Self::Boolean(_) => Type::boolean(),
            Self::Integer(inner) => inner.r#type(),
            Self::Range(inner) => inner.r#type(),
            Self::RangeInclusive(inner) => inner.r#type(),
            Self::String(_) => Type::string(),
        }
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Unit, Self::Unit) => true,
            (Self::Boolean(_), Self::Boolean(_)) => true,
            (Self::Integer(inner_1), Self::Integer(inner_2)) => {
                inner_1.has_the_same_type_as(inner_2)
            }
            (Self::Range(inner_1), Self::Range(inner_2)) => inner_1.has_the_same_type_as(inner_2),
            (Self::RangeInclusive(inner_1), Self::RangeInclusive(inner_2)) => {
                inner_1.has_the_same_type_as(inner_2)
            }
            _ => false,
        }
    }

    pub fn range_inclusive(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .range_inclusive(integer_2)
                    .map(Self::RangeInclusive)
                    .map_err(Error::Integer),
                value => Err(Error::OperatorRangeInclusiveSecondOperandExpectedInteger(
                    value.to_string(),
                )),
            },
            value => Err(Error::OperatorRangeInclusiveFirstOperandExpectedInteger(
                value.to_string(),
            )),
        }
    }

    pub fn range(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .range(integer_2)
                    .map(Self::Range)
                    .map_err(Error::Integer),
                value => Err(Error::OperatorRangeSecondOperandExpectedInteger(
                    value.to_string(),
                )),
            },
            value => Err(Error::OperatorRangeFirstOperandExpectedInteger(
                value.to_string(),
            )),
        }
    }

    pub fn or(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Boolean(value_1) => match other {
                Self::Boolean(value_2) => {
                    let result = *value_1 || *value_2;
                    Ok(Self::Boolean(result))
                }
                constant => Err(Error::OperatorOrSecondOperandExpectedBoolean(
                    constant.to_string(),
                )),
            },
            constant => Err(Error::OperatorOrFirstOperandExpectedBoolean(
                constant.to_string(),
            )),
        }
    }

    pub fn xor(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Boolean(value_1) => match other {
                Self::Boolean(value_2) => {
                    let result = !*value_1 && *value_2 || *value_1 && !*value_2;
                    Ok(Self::Boolean(result))
                }
                constant => Err(Error::OperatorXorSecondOperandExpectedBoolean(
                    constant.to_string(),
                )),
            },
            constant => Err(Error::OperatorXorFirstOperandExpectedBoolean(
                constant.to_string(),
            )),
        }
    }

    pub fn and(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Boolean(value_1) => match other {
                Self::Boolean(value_2) => {
                    let result = *value_1 && *value_2;
                    Ok(Self::Boolean(result))
                }
                constant => Err(Error::OperatorAndSecondOperandExpectedBoolean(
                    constant.to_string(),
                )),
            },
            constant => Err(Error::OperatorAndFirstOperandExpectedBoolean(
                constant.to_string(),
            )),
        }
    }

    pub fn equals(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Self::Unit, Self::Unit) => Ok(Self::Boolean(true)),
            (Self::Unit, value_2) => Err(Error::OperatorEqualsSecondOperandExpectedUnit(
                value_2.to_string(),
            )),
            (Self::Boolean(value_1), Self::Boolean(value_2)) => {
                let result = value_1 == value_2;
                Ok(Self::Boolean(result))
            }
            (Self::Boolean(_), value_2) => Err(Error::OperatorEqualsSecondOperandExpectedBoolean(
                value_2.to_string(),
            )),
            (Self::Integer(value_1), Self::Integer(value_2)) => {
                let result = value_1.equals(value_2).map_err(Error::Integer)?;
                Ok(Self::Boolean(result))
            }
            (Self::Integer(_), value_2) => Err(Error::OperatorEqualsSecondOperandExpectedInteger(
                value_2.to_string(),
            )),
            (value_1, _) => Err(Error::OperatorEqualsFirstOperandExpectedPrimitiveType(
                value_1.to_string(),
            )),
        }
    }

    pub fn not_equals(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Self::Unit, Self::Unit) => Ok(Self::Boolean(true)),
            (Self::Unit, value_2) => Err(Error::OperatorNotEqualsSecondOperandExpectedUnit(
                value_2.to_string(),
            )),
            (Self::Boolean(value_1), Self::Boolean(value_2)) => {
                let result = value_1 != value_2;
                Ok(Self::Boolean(result))
            }
            (Self::Boolean(_), value_2) => Err(
                Error::OperatorNotEqualsSecondOperandExpectedBoolean(value_2.to_string()),
            ),
            (Self::Integer(value_1), Self::Integer(value_2)) => {
                let result = value_1.not_equals(value_2).map_err(Error::Integer)?;
                Ok(Self::Boolean(result))
            }
            (Self::Integer(_), value_2) => Err(
                Error::OperatorNotEqualsSecondOperandExpectedInteger(value_2.to_string()),
            ),
            (value_1, _) => Err(Error::OperatorNotEqualsFirstOperandExpectedPrimitiveType(
                value_1.to_string(),
            )),
        }
    }

    pub fn greater_equals(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .greater_equals(integer_2)
                    .map(Self::Boolean)
                    .map_err(Error::Integer),
                value => Err(Error::OperatorGreaterEqualsSecondOperandExpectedInteger(
                    value.to_string(),
                )),
            },
            value => Err(Error::OperatorGreaterEqualsFirstOperandExpectedInteger(
                value.to_string(),
            )),
        }
    }

    pub fn lesser_equals(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .lesser_equals(integer_2)
                    .map(Self::Boolean)
                    .map_err(Error::Integer),
                value => Err(Error::OperatorLesserEqualsSecondOperandExpectedInteger(
                    value.to_string(),
                )),
            },
            value => Err(Error::OperatorLesserEqualsFirstOperandExpectedInteger(
                value.to_string(),
            )),
        }
    }

    pub fn greater(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .greater(integer_2)
                    .map(Self::Boolean)
                    .map_err(Error::Integer),
                value => Err(Error::OperatorGreaterSecondOperandExpectedInteger(
                    value.to_string(),
                )),
            },
            value => Err(Error::OperatorGreaterFirstOperandExpectedInteger(
                value.to_string(),
            )),
        }
    }

    pub fn lesser(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .lesser(integer_2)
                    .map(Self::Boolean)
                    .map_err(Error::Integer),
                value => Err(Error::OperatorLesserSecondOperandExpectedInteger(
                    value.to_string(),
                )),
            },
            value => Err(Error::OperatorLesserFirstOperandExpectedInteger(
                value.to_string(),
            )),
        }
    }

    pub fn add(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .add(integer_2)
                    .map(Self::Integer)
                    .map_err(Error::Integer),
                value => Err(Error::OperatorAdditionSecondOperandExpectedInteger(
                    value.to_string(),
                )),
            },
            value => Err(Error::OperatorAdditionFirstOperandExpectedInteger(
                value.to_string(),
            )),
        }
    }

    pub fn subtract(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .subtract(integer_2)
                    .map(Self::Integer)
                    .map_err(Error::Integer),
                value => Err(Error::OperatorSubtractionSecondOperandExpectedInteger(
                    value.to_string(),
                )),
            },
            value => Err(Error::OperatorSubtractionFirstOperandExpectedInteger(
                value.to_string(),
            )),
        }
    }

    pub fn multiply(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .multiply(integer_2)
                    .map(Self::Integer)
                    .map_err(Error::Integer),
                value => Err(Error::OperatorMultiplicationSecondOperandExpectedInteger(
                    value.to_string(),
                )),
            },
            value => Err(Error::OperatorMultiplicationFirstOperandExpectedInteger(
                value.to_string(),
            )),
        }
    }

    pub fn divide(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .divide(integer_2)
                    .map(Self::Integer)
                    .map_err(Error::Integer),
                value => Err(Error::OperatorDivisionSecondOperandExpectedInteger(
                    value.to_string(),
                )),
            },
            value => Err(Error::OperatorDivisionFirstOperandExpectedInteger(
                value.to_string(),
            )),
        }
    }

    pub fn remainder(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .remainder(integer_2)
                    .map(Self::Integer)
                    .map_err(Error::Integer),
                value => Err(Error::OperatorRemainderSecondOperandExpectedInteger(
                    value.to_string(),
                )),
            },
            value => Err(Error::OperatorRemainderFirstOperandExpectedInteger(
                value.to_string(),
            )),
        }
    }

    pub fn negate(&self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer) => integer.negate().map(Self::Integer).map_err(Error::Integer),
            value => Err(Error::OperatorNegationExpectedInteger(value.to_string())),
        }
    }

    pub fn not(&self) -> Result<Self, Error> {
        match self {
            Self::Boolean(value) => {
                let result = !value;
                Ok(Self::Boolean(result))
            }
            value => Err(Error::OperatorNotExpectedBoolean(value.to_string())),
        }
    }

    pub fn cast(&mut self, to: &Type) -> Result<Option<(bool, usize)>, Error> {
        let from = self.r#type();
        Caster::cast(&from, &to).map_err(Error::Casting)?;

        let (is_signed, bitlength) = match to {
            Type::IntegerUnsigned { bitlength } => (false, *bitlength),
            Type::IntegerSigned { bitlength } => (true, *bitlength),
            Type::Field => (false, crate::BITLENGTH_FIELD),
            _ => return Ok(None),
        };

        if let Self::Integer(integer) = self {
            integer.cast(is_signed, bitlength).map_err(Error::Integer)?;
        }
        Ok(Some((is_signed, bitlength)))
    }

    pub fn to_instruction(&self) -> Instruction {
        match self {
            Self::Boolean(boolean) => Instruction::PushConst(zinc_bytecode::PushConst::new(
                if *boolean {
                    BigInt::one()
                } else {
                    BigInt::zero()
                },
                ScalarType::Boolean,
            )),
            Self::Integer(integer) => integer.to_instruction(),
            Self::Unit => unreachable!(),
            Self::Range(_) => unreachable!(),
            Self::RangeInclusive(_) => unreachable!(),
            Self::String(_) => unreachable!(),
        }
    }
}

impl From<bool> for Constant {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<BooleanLiteral> for Constant {
    fn from(value: BooleanLiteral) -> Self {
        let value: bool = value.into();
        Self::from(value)
    }
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unit => write!(f, "()"),
            Self::Boolean(inner) => write!(f, "{}", inner),
            Self::Integer(inner) => write!(f, "{}", inner),
            Self::Range(inner) => write!(f, "{}", inner),
            Self::RangeInclusive(inner) => write!(f, "{}", inner),
            Self::String(inner) => write!(f, "{}", inner),
        }
    }
}
