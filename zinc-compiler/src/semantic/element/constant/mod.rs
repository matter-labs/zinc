//!
//! The semantic analyzer constant element.
//!

mod tests;

pub mod array;
pub mod boolean;
pub mod error;
pub mod integer;
pub mod range;
pub mod range_inclusive;
pub mod structure;
pub mod tuple;

use std::fmt;

use crate::generator::expression::operator::Operator as GeneratorExpressionOperator;
use crate::semantic::casting::Caster;
use crate::semantic::element::access::Field as FieldAccess;
use crate::semantic::element::access::Index as IndexAccess;
use crate::semantic::element::r#type::Type;

use self::array::Array;
use self::boolean::Boolean;
use self::error::Error;
use self::integer::Integer;
use self::range::Range;
use self::range_inclusive::RangeInclusive;
use self::structure::Structure;
use self::tuple::Tuple;

///
/// Constants are parts of a constant expression.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    Unit,
    Boolean(Boolean),
    Integer(Integer),
    Range(Range),
    RangeInclusive(RangeInclusive),
    String(String),
    Array(Array),
    Tuple(Tuple),
    Structure(Structure),
}

impl Constant {
    pub fn r#type(&self) -> Type {
        match self {
            Self::Unit => Type::unit(),
            Self::Boolean(inner) => inner.r#type(),
            Self::Integer(inner) => inner.r#type(),
            Self::Range(inner) => inner.r#type(),
            Self::RangeInclusive(inner) => inner.r#type(),
            Self::String(_) => Type::string(),
            Self::Array(inner) => inner.r#type(),
            Self::Tuple(inner) => inner.r#type(),
            Self::Structure(inner) => inner.r#type(),
        }
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Unit, Self::Unit) => true,
            (Self::Boolean(inner_1), Self::Boolean(inner_2)) => {
                inner_1.has_the_same_type_as(inner_2)
            }
            (Self::Integer(inner_1), Self::Integer(inner_2)) => {
                inner_1.has_the_same_type_as(inner_2)
            }
            (Self::Range(inner_1), Self::Range(inner_2)) => inner_1.has_the_same_type_as(inner_2),
            (Self::RangeInclusive(inner_1), Self::RangeInclusive(inner_2)) => {
                inner_1.has_the_same_type_as(inner_2)
            }
            (Self::Array(inner_1), Self::Array(inner_2)) => inner_1.has_the_same_type_as(inner_2),
            (Self::Tuple(inner_1), Self::Tuple(inner_2)) => inner_1.has_the_same_type_as(inner_2),
            (Self::Structure(inner_1), Self::Structure(inner_2)) => {
                inner_1.has_the_same_type_as(inner_2)
            }
            _ => false,
        }
    }

    pub fn range_inclusive(self, other: Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .range_inclusive(integer_2)
                    .map(Self::RangeInclusive)
                    .map_err(Error::Integer),
                constant => Err(Error::OperatorRangeInclusiveSecondOperandExpectedInteger {
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorRangeInclusiveFirstOperandExpectedInteger {
                found: constant.to_string(),
            }),
        }
    }

    pub fn range(self, other: Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .range(integer_2)
                    .map(Self::Range)
                    .map_err(Error::Integer),
                constant => Err(Error::OperatorRangeSecondOperandExpectedInteger {
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorRangeFirstOperandExpectedInteger {
                found: constant.to_string(),
            }),
        }
    }

    pub fn or(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Boolean(constant_1) => match other {
                Self::Boolean(constant_2) => Ok((
                    Self::Boolean(constant_1.or(constant_2)),
                    GeneratorExpressionOperator::Or,
                )),
                constant => Err(Error::OperatorOrSecondOperandExpectedBoolean {
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorOrFirstOperandExpectedBoolean {
                found: constant.to_string(),
            }),
        }
    }

    pub fn xor(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Boolean(constant_1) => match other {
                Self::Boolean(constant_2) => Ok((
                    Self::Boolean(constant_1.xor(constant_2)),
                    GeneratorExpressionOperator::Xor,
                )),
                constant => Err(Error::OperatorXorSecondOperandExpectedBoolean {
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorXorFirstOperandExpectedBoolean {
                found: constant.to_string(),
            }),
        }
    }

    pub fn and(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Boolean(constant_1) => match other {
                Self::Boolean(constant_2) => Ok((
                    Self::Boolean(constant_1.and(constant_2)),
                    GeneratorExpressionOperator::And,
                )),
                constant => Err(Error::OperatorAndSecondOperandExpectedBoolean {
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorAndFirstOperandExpectedBoolean {
                found: constant.to_string(),
            }),
        }
    }

    pub fn equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Self::Unit, Self::Unit) => Ok((
                Self::Boolean(Boolean::new(true)),
                GeneratorExpressionOperator::Equals,
            )),
            (Self::Unit, constant_2) => Err(Error::OperatorEqualsSecondOperandExpectedUnit {
                found: constant_2.to_string(),
            }),
            (Self::Boolean(constant_1), Self::Boolean(constant_2)) => Ok((
                Self::Boolean(constant_1.equals(constant_2)),
                GeneratorExpressionOperator::Equals,
            )),
            (Self::Boolean(_), constant_2) => {
                Err(Error::OperatorEqualsSecondOperandExpectedBoolean {
                    found: constant_2.to_string(),
                })
            }
            (Self::Integer(constant_1), Self::Integer(constant_2)) => constant_1
                .equals(constant_2)
                .map(|(boolean, operator)| (Self::Boolean(boolean), operator))
                .map_err(Error::Integer),
            (Self::Integer(_), constant_2) => {
                Err(Error::OperatorEqualsSecondOperandExpectedInteger {
                    found: constant_2.to_string(),
                })
            }
            (constant_1, _) => Err(Error::OperatorEqualsFirstOperandExpectedPrimitiveType {
                found: constant_1.to_string(),
            }),
        }
    }

    pub fn not_equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Self::Unit, Self::Unit) => Ok((
                Self::Boolean(Boolean::new(false)),
                GeneratorExpressionOperator::NotEquals,
            )),
            (Self::Unit, constant_2) => Err(Error::OperatorNotEqualsSecondOperandExpectedUnit {
                found: constant_2.to_string(),
            }),
            (Self::Boolean(constant_1), Self::Boolean(constant_2)) => Ok((
                Self::Boolean(constant_1.not_equals(constant_2)),
                GeneratorExpressionOperator::NotEquals,
            )),
            (Self::Boolean(_), constant_2) => {
                Err(Error::OperatorNotEqualsSecondOperandExpectedBoolean {
                    found: constant_2.to_string(),
                })
            }
            (Self::Integer(constant_1), Self::Integer(constant_2)) => constant_1
                .not_equals(constant_2)
                .map(|(boolean, operator)| (Self::Boolean(boolean), operator))
                .map_err(Error::Integer),
            (Self::Integer(_), constant_2) => {
                Err(Error::OperatorNotEqualsSecondOperandExpectedInteger {
                    found: constant_2.to_string(),
                })
            }
            (constant_1, _) => Err(Error::OperatorNotEqualsFirstOperandExpectedPrimitiveType {
                found: constant_1.to_string(),
            }),
        }
    }

    pub fn greater_equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .greater_equals(integer_2)
                    .map(|(boolean, operator)| (Self::Boolean(boolean), operator))
                    .map_err(Error::Integer),
                constant => Err(Error::OperatorGreaterEqualsSecondOperandExpectedInteger {
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorGreaterEqualsFirstOperandExpectedInteger {
                found: constant.to_string(),
            }),
        }
    }

    pub fn lesser_equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .lesser_equals(integer_2)
                    .map(|(boolean, operator)| (Self::Boolean(boolean), operator))
                    .map_err(Error::Integer),
                constant => Err(Error::OperatorLesserEqualsSecondOperandExpectedInteger {
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorLesserEqualsFirstOperandExpectedInteger {
                found: constant.to_string(),
            }),
        }
    }

    pub fn greater(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .greater(integer_2)
                    .map(|(boolean, operator)| (Self::Boolean(boolean), operator))
                    .map_err(Error::Integer),
                constant => Err(Error::OperatorGreaterSecondOperandExpectedInteger {
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorGreaterFirstOperandExpectedInteger {
                found: constant.to_string(),
            }),
        }
    }

    pub fn lesser(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .lesser(integer_2)
                    .map(|(boolean, operator)| (Self::Boolean(boolean), operator))
                    .map_err(Error::Integer),
                constant => Err(Error::OperatorLesserSecondOperandExpectedInteger {
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorLesserFirstOperandExpectedInteger {
                found: constant.to_string(),
            }),
        }
    }

    pub fn bitwise_or(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .bitwise_or(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator))
                    .map_err(Error::Integer),
                constant => Err(Error::OperatorBitwiseOrSecondOperandExpectedInteger {
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorBitwiseOrFirstOperandExpectedInteger {
                found: constant.to_string(),
            }),
        }
    }

    pub fn bitwise_xor(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .bitwise_xor(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator))
                    .map_err(Error::Integer),
                constant => Err(Error::OperatorBitwiseXorSecondOperandExpectedInteger {
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorBitwiseXorFirstOperandExpectedInteger {
                found: constant.to_string(),
            }),
        }
    }

    pub fn bitwise_and(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .bitwise_and(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator))
                    .map_err(Error::Integer),
                constant => Err(Error::OperatorBitwiseAndSecondOperandExpectedInteger {
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorBitwiseAndFirstOperandExpectedInteger {
                found: constant.to_string(),
            }),
        }
    }

    pub fn bitwise_shift_left(
        self,
        other: Self,
    ) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .bitwise_shift_left(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator))
                    .map_err(Error::Integer),
                constant => Err(
                    Error::OperatorBitwiseShiftLeftSecondOperandExpectedInteger {
                        found: constant.to_string(),
                    },
                ),
            },
            constant => Err(Error::OperatorBitwiseShiftLeftFirstOperandExpectedInteger {
                found: constant.to_string(),
            }),
        }
    }

    pub fn bitwise_shift_right(
        self,
        other: Self,
    ) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .bitwise_shift_right(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator))
                    .map_err(Error::Integer),
                constant => Err(
                    Error::OperatorBitwiseShiftRightSecondOperandExpectedInteger {
                        found: constant.to_string(),
                    },
                ),
            },
            constant => Err(
                Error::OperatorBitwiseShiftRightFirstOperandExpectedInteger {
                    found: constant.to_string(),
                },
            ),
        }
    }

    pub fn add(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .add(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator))
                    .map_err(Error::Integer),
                constant => Err(Error::OperatorAdditionSecondOperandExpectedInteger {
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorAdditionFirstOperandExpectedInteger {
                found: constant.to_string(),
            }),
        }
    }

    pub fn subtract(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .subtract(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator))
                    .map_err(Error::Integer),
                constant => Err(Error::OperatorSubtractionSecondOperandExpectedInteger {
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorSubtractionFirstOperandExpectedInteger {
                found: constant.to_string(),
            }),
        }
    }

    pub fn multiply(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .multiply(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator))
                    .map_err(Error::Integer),
                constant => Err(Error::OperatorMultiplicationSecondOperandExpectedInteger {
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorMultiplicationFirstOperandExpectedInteger {
                found: constant.to_string(),
            }),
        }
    }

    pub fn divide(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .divide(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator))
                    .map_err(Error::Integer),
                constant => Err(Error::OperatorDivisionSecondOperandExpectedInteger {
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorDivisionFirstOperandExpectedInteger {
                found: constant.to_string(),
            }),
        }
    }

    pub fn remainder(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .remainder(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator))
                    .map_err(Error::Integer),
                constant => Err(Error::OperatorRemainderSecondOperandExpectedInteger {
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorRemainderFirstOperandExpectedInteger {
                found: constant.to_string(),
            }),
        }
    }

    pub fn cast(self, to: Type) -> Result<(Self, Option<GeneratorExpressionOperator>), Error> {
        let from = self.r#type();
        Caster::cast(&from, &to).map_err(Error::Casting)?;

        let (is_signed, bitlength) = match to {
            Type::IntegerUnsigned { bitlength } => (false, bitlength),
            Type::IntegerSigned { bitlength } => (true, bitlength),
            Type::Field => (false, crate::BITLENGTH_FIELD),
            _ => return Ok((self, None)),
        };

        Ok(match self {
            Self::Integer(integer) => integer
                .cast(is_signed, bitlength)
                .map(|(integer, operator)| (Self::Integer(integer), operator))
                .map_err(Error::Integer)?,
            operand => (operand, None),
        })
    }

    pub fn not(self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Boolean(constant) => Ok((
                Self::Boolean(constant.not()),
                GeneratorExpressionOperator::Not,
            )),
            constant => Err(Error::OperatorNotExpectedBoolean {
                found: constant.to_string(),
            }),
        }
    }

    pub fn bitwise_not(self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer) => integer
                .bitwise_not()
                .map(|(integer, operator)| (Self::Integer(integer), operator))
                .map_err(Error::Integer),
            constant => Err(Error::OperatorBitwiseNotExpectedInteger {
                found: constant.to_string(),
            }),
        }
    }

    pub fn negate(self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer) => integer
                .negate()
                .map(|(integer, operator)| (Self::Integer(integer), operator))
                .map_err(Error::Integer),
            constant => Err(Error::OperatorNegationExpectedInteger {
                found: constant.to_string(),
            }),
        }
    }

    pub fn index(self, other: Constant) -> Result<(Self, IndexAccess), Error> {
        match self {
            Constant::Array(array) => match other {
                Constant::Integer(integer) => {
                    array.slice_single(integer.value).map_err(Error::Array)
                }
                Constant::Range(range) => array
                    .slice_range(range.start, range.end)
                    .map(|(constant, access)| (constant, access))
                    .map_err(Error::Array),
                Constant::RangeInclusive(range) => array
                    .slice_range_inclusive(range.start, range.end)
                    .map(|(constant, access)| (constant, access))
                    .map_err(Error::Array),
                constant => Err(Error::OperatorIndexSecondOperandExpectedIntegerOrRange {
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorIndexFirstOperandExpectedArray {
                found: constant.to_string(),
            }),
        }
    }

    pub fn field_tuple(self, field_index: usize) -> Result<(Self, FieldAccess), Error> {
        match self {
            Constant::Tuple(tuple) => tuple.slice(field_index).map_err(Error::Tuple),
            constant => Err(Error::OperatorFieldFirstOperandExpectedTuple {
                found: constant.to_string(),
            }),
        }
    }

    pub fn field_structure(self, field_name: String) -> Result<(Self, FieldAccess), Error> {
        match self {
            Constant::Structure(structure) => structure.slice(field_name).map_err(Error::Structure),
            constant => Err(Error::OperatorFieldFirstOperandExpectedStructure {
                found: constant.to_string(),
            }),
        }
    }
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unit => write!(f, "unit constant '()'"),
            Self::Boolean(inner) => write!(f, "{}", inner),
            Self::Integer(inner) => write!(f, "{}", inner),
            Self::Range(inner) => write!(f, "{}", inner),
            Self::RangeInclusive(inner) => write!(f, "{}", inner),
            Self::String(constant) => write!(f, "string constant '{}'", constant),
            Self::Array(inner) => write!(f, "{}", inner),
            Self::Tuple(inner) => write!(f, "{}", inner),
            Self::Structure(inner) => write!(f, "{}", inner),
        }
    }
}
