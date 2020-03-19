//!
//! The semantic analyzer constant element.
//!

mod tests;

pub mod error;
pub mod integer;
pub mod range;
pub mod range_inclusive;

use std::fmt;

use num_bigint::BigInt;
use num_traits::One;
use num_traits::Zero;

use zinc_bytecode::scalar::ScalarType;
use zinc_bytecode::Instruction;

use crate::generator::expression::operand::constant::Constant as GeneratorConstant;
use crate::generator::expression::operand::Operand as GeneratorOperand;
use crate::semantic::caster::Caster;
use crate::semantic::element::r#type::Type;
use crate::syntax::BooleanLiteral;

use self::error::Error;
use self::integer::Integer;
use self::range::Range;
use self::range_inclusive::RangeInclusive;

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
                value => Err(Error::OperatorRangeInclusiveSecondOperandExpectedInteger {
                    found: value.to_string(),
                }),
            },
            value => Err(Error::OperatorRangeInclusiveFirstOperandExpectedInteger {
                found: value.to_string(),
            }),
        }
    }

    pub fn range(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .range(integer_2)
                    .map(Self::Range)
                    .map_err(Error::Integer),
                value => Err(Error::OperatorRangeSecondOperandExpectedInteger {
                    found: value.to_string(),
                }),
            },
            value => Err(Error::OperatorRangeFirstOperandExpectedInteger {
                found: value.to_string(),
            }),
        }
    }

    pub fn or(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Boolean(value_1) => match other {
                Self::Boolean(value_2) => {
                    let result = *value_1 || *value_2;
                    Ok(Self::Boolean(result))
                }
                constant => Err(Error::OperatorOrSecondOperandExpectedBoolean {
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorOrFirstOperandExpectedBoolean {
                found: constant.to_string(),
            }),
        }
    }

    pub fn xor(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Boolean(value_1) => match other {
                Self::Boolean(value_2) => {
                    let result = !*value_1 && *value_2 || *value_1 && !*value_2;
                    Ok(Self::Boolean(result))
                }
                constant => Err(Error::OperatorXorSecondOperandExpectedBoolean {
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorXorFirstOperandExpectedBoolean {
                found: constant.to_string(),
            }),
        }
    }

    pub fn and(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Boolean(value_1) => match other {
                Self::Boolean(value_2) => {
                    let result = *value_1 && *value_2;
                    Ok(Self::Boolean(result))
                }
                constant => Err(Error::OperatorAndSecondOperandExpectedBoolean {
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorAndFirstOperandExpectedBoolean {
                found: constant.to_string(),
            }),
        }
    }

    pub fn equals(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Self::Unit, Self::Unit) => Ok(Self::Boolean(true)),
            (Self::Unit, value_2) => Err(Error::OperatorEqualsSecondOperandExpectedUnit {
                found: value_2.to_string(),
            }),
            (Self::Boolean(value_1), Self::Boolean(value_2)) => {
                let result = value_1 == value_2;
                Ok(Self::Boolean(result))
            }
            (Self::Boolean(_), value_2) => Err(Error::OperatorEqualsSecondOperandExpectedBoolean {
                found: value_2.to_string(),
            }),
            (Self::Integer(value_1), Self::Integer(value_2)) => {
                let result = value_1.equals(value_2).map_err(Error::Integer)?;
                Ok(Self::Boolean(result))
            }
            (Self::Integer(_), value_2) => Err(Error::OperatorEqualsSecondOperandExpectedInteger {
                found: value_2.to_string(),
            }),
            (value_1, _) => Err(Error::OperatorEqualsFirstOperandExpectedPrimitiveType {
                found: value_1.to_string(),
            }),
        }
    }

    pub fn not_equals(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Self::Unit, Self::Unit) => Ok(Self::Boolean(true)),
            (Self::Unit, value_2) => Err(Error::OperatorNotEqualsSecondOperandExpectedUnit {
                found: value_2.to_string(),
            }),
            (Self::Boolean(value_1), Self::Boolean(value_2)) => {
                let result = value_1 != value_2;
                Ok(Self::Boolean(result))
            }
            (Self::Boolean(_), value_2) => {
                Err(Error::OperatorNotEqualsSecondOperandExpectedBoolean {
                    found: value_2.to_string(),
                })
            }
            (Self::Integer(value_1), Self::Integer(value_2)) => {
                let result = value_1.not_equals(value_2).map_err(Error::Integer)?;
                Ok(Self::Boolean(result))
            }
            (Self::Integer(_), value_2) => {
                Err(Error::OperatorNotEqualsSecondOperandExpectedInteger {
                    found: value_2.to_string(),
                })
            }
            (value_1, _) => Err(Error::OperatorNotEqualsFirstOperandExpectedPrimitiveType {
                found: value_1.to_string(),
            }),
        }
    }

    pub fn greater_equals(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .greater_equals(integer_2)
                    .map(Self::Boolean)
                    .map_err(Error::Integer),
                value => Err(Error::OperatorGreaterEqualsSecondOperandExpectedInteger {
                    found: value.to_string(),
                }),
            },
            value => Err(Error::OperatorGreaterEqualsFirstOperandExpectedInteger {
                found: value.to_string(),
            }),
        }
    }

    pub fn lesser_equals(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .lesser_equals(integer_2)
                    .map(Self::Boolean)
                    .map_err(Error::Integer),
                value => Err(Error::OperatorLesserEqualsSecondOperandExpectedInteger {
                    found: value.to_string(),
                }),
            },
            value => Err(Error::OperatorLesserEqualsFirstOperandExpectedInteger {
                found: value.to_string(),
            }),
        }
    }

    pub fn greater(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .greater(integer_2)
                    .map(Self::Boolean)
                    .map_err(Error::Integer),
                value => Err(Error::OperatorGreaterSecondOperandExpectedInteger {
                    found: value.to_string(),
                }),
            },
            value => Err(Error::OperatorGreaterFirstOperandExpectedInteger {
                found: value.to_string(),
            }),
        }
    }

    pub fn lesser(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .lesser(integer_2)
                    .map(Self::Boolean)
                    .map_err(Error::Integer),
                value => Err(Error::OperatorLesserSecondOperandExpectedInteger {
                    found: value.to_string(),
                }),
            },
            value => Err(Error::OperatorLesserFirstOperandExpectedInteger {
                found: value.to_string(),
            }),
        }
    }

    pub fn bitwise_or(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .bitwise_or(integer_2)
                    .map(|_| Self::Integer(integer_1.to_owned()))
                    .map_err(Error::Integer),
                value => Err(Error::OperatorBitwiseOrSecondOperandExpectedInteger {
                    found: value.to_string(),
                }),
            },
            value => Err(Error::OperatorBitwiseOrFirstOperandExpectedInteger {
                found: value.to_string(),
            }),
        }
    }

    pub fn bitwise_xor(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .bitwise_xor(integer_2)
                    .map(|_| Self::Integer(integer_1.to_owned()))
                    .map_err(Error::Integer),
                value => Err(Error::OperatorBitwiseXorSecondOperandExpectedInteger {
                    found: value.to_string(),
                }),
            },
            value => Err(Error::OperatorBitwiseXorFirstOperandExpectedInteger {
                found: value.to_string(),
            }),
        }
    }

    pub fn bitwise_and(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .bitwise_and(integer_2)
                    .map(|_| Self::Integer(integer_1.to_owned()))
                    .map_err(Error::Integer),
                value => Err(Error::OperatorBitwiseAndSecondOperandExpectedInteger {
                    found: value.to_string(),
                }),
            },
            value => Err(Error::OperatorBitwiseAndFirstOperandExpectedInteger {
                found: value.to_string(),
            }),
        }
    }

    pub fn bitwise_shift_left(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .bitwise_shift_left(integer_2)
                    .map(|_| Self::Integer(integer_1.to_owned()))
                    .map_err(Error::Integer),
                value => Err(
                    Error::OperatorBitwiseShiftLeftSecondOperandExpectedInteger {
                        found: value.to_string(),
                    },
                ),
            },
            value => Err(Error::OperatorBitwiseShiftLeftFirstOperandExpectedInteger {
                found: value.to_string(),
            }),
        }
    }

    pub fn bitwise_shift_right(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .bitwise_shift_right(integer_2)
                    .map(|_| Self::Integer(integer_1.to_owned()))
                    .map_err(Error::Integer),
                value => Err(
                    Error::OperatorBitwiseShiftRightSecondOperandExpectedInteger {
                        found: value.to_string(),
                    },
                ),
            },
            value => Err(
                Error::OperatorBitwiseShiftRightFirstOperandExpectedInteger {
                    found: value.to_string(),
                },
            ),
        }
    }

    pub fn add(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .add(integer_2)
                    .map(Self::Integer)
                    .map_err(Error::Integer),
                value => Err(Error::OperatorAdditionSecondOperandExpectedInteger {
                    found: value.to_string(),
                }),
            },
            value => Err(Error::OperatorAdditionFirstOperandExpectedInteger {
                found: value.to_string(),
            }),
        }
    }

    pub fn subtract(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .subtract(integer_2)
                    .map(Self::Integer)
                    .map_err(Error::Integer),
                value => Err(Error::OperatorSubtractionSecondOperandExpectedInteger {
                    found: value.to_string(),
                }),
            },
            value => Err(Error::OperatorSubtractionFirstOperandExpectedInteger {
                found: value.to_string(),
            }),
        }
    }

    pub fn multiply(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .multiply(integer_2)
                    .map(Self::Integer)
                    .map_err(Error::Integer),
                value => Err(Error::OperatorMultiplicationSecondOperandExpectedInteger {
                    found: value.to_string(),
                }),
            },
            value => Err(Error::OperatorMultiplicationFirstOperandExpectedInteger {
                found: value.to_string(),
            }),
        }
    }

    pub fn divide(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .divide(integer_2)
                    .map(Self::Integer)
                    .map_err(Error::Integer),
                value => Err(Error::OperatorDivisionSecondOperandExpectedInteger {
                    found: value.to_string(),
                }),
            },
            value => Err(Error::OperatorDivisionFirstOperandExpectedInteger {
                found: value.to_string(),
            }),
        }
    }

    pub fn remainder(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .remainder(integer_2)
                    .map(Self::Integer)
                    .map_err(Error::Integer),
                value => Err(Error::OperatorRemainderSecondOperandExpectedInteger {
                    found: value.to_string(),
                }),
            },
            value => Err(Error::OperatorRemainderFirstOperandExpectedInteger {
                found: value.to_string(),
            }),
        }
    }

    pub fn not(&self) -> Result<Self, Error> {
        match self {
            Self::Boolean(value) => {
                let result = !value;
                Ok(Self::Boolean(result))
            }
            value => Err(Error::OperatorNotExpectedBoolean {
                found: value.to_string(),
            }),
        }
    }

    pub fn bitwise_not(&self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer) => integer
                .bitwise_not()
                .map(Self::Integer)
                .map_err(Error::Integer),
            value => Err(Error::OperatorBitwiseNotExpectedInteger {
                found: value.to_string(),
            }),
        }
    }

    pub fn negate(&self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer) => integer.negate().map(Self::Integer).map_err(Error::Integer),
            value => Err(Error::OperatorNegationExpectedInteger {
                found: value.to_string(),
            }),
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

    pub fn to_intermediate(&self) -> GeneratorOperand {
        match self {
            Self::Boolean(boolean) => GeneratorOperand::Constant(GeneratorConstant::new(
                if *boolean {
                    BigInt::one()
                } else {
                    BigInt::zero()
                },
                false,
                crate::BITLENGTH_BOOLEAN,
            )),
            Self::Integer(integer) => integer.to_intermediate(),
            Self::Unit => unreachable!(),
            Self::Range(_) => unreachable!(),
            Self::RangeInclusive(_) => unreachable!(),
            Self::String(_) => unreachable!(),
        }
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
            Self::Unit => write!(f, "unit constant '()'"),
            Self::Boolean(constant) => write!(f, "boolean constant '{}'", constant),
            Self::Integer(inner) => write!(f, "{}", inner),
            Self::Range(inner) => write!(f, "{}", inner),
            Self::RangeInclusive(inner) => write!(f, "{}", inner),
            Self::String(constant) => write!(f, "string constant '{}'", constant),
        }
    }
}
