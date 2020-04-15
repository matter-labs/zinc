//!
//! The semantic analyzer value element.
//!

mod tests;

pub mod array;
pub mod error;
pub mod integer;
pub mod structure;
pub mod tuple;

use std::convert::TryFrom;
use std::fmt;

use crate::generator::expression::operator::Operator as GeneratorExpressionOperator;
use crate::semantic::casting::Caster;
use crate::semantic::element::access::Field as FieldAccess;
use crate::semantic::element::access::Index as IndexAccess;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::Type;

use self::array::Array;
use self::error::Error;
use self::integer::Integer;
use self::structure::Structure;
use self::tuple::Tuple;

///
/// Value are parts of a non-constant expression.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Unit,
    Boolean,
    Integer(Integer),
    Array(Array),
    Tuple(Tuple),
    Structure(Structure),
}

impl Value {
    pub fn r#type(&self) -> Type {
        match self {
            Self::Unit => Type::unit(),
            Self::Boolean => Type::boolean(),
            Self::Integer(integer) => integer.r#type(),
            Self::Array(array) => array.r#type(),
            Self::Tuple(tuple) => tuple.r#type(),
            Self::Structure(structure) => structure.r#type(),
        }
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Unit, Self::Unit) => true,
            (Self::Boolean, Self::Boolean) => true,
            (Self::Integer(value_1), Self::Integer(value_2)) => {
                value_1.has_the_same_type_as(value_2)
            }
            (Self::Array(value_1), Self::Array(value_2)) => value_1.has_the_same_type_as(value_2),
            (Self::Tuple(value_1), Self::Tuple(value_2)) => value_1.has_the_same_type_as(value_2),
            (Self::Structure(value_1), Self::Structure(value_2)) => {
                value_1.has_the_same_type_as(value_2)
            }
            _ => false,
        }
    }

    pub fn or(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Boolean => match other {
                Self::Boolean => Ok((Self::Boolean, GeneratorExpressionOperator::Or)),
                value => Err(Error::OperatorOrSecondOperandExpectedBoolean {
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorOrFirstOperandExpectedBoolean {
                found: value.r#type().to_string(),
            }),
        }
    }

    pub fn xor(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Boolean => match other {
                Self::Boolean => Ok((Self::Boolean, GeneratorExpressionOperator::Xor)),
                value => Err(Error::OperatorXorSecondOperandExpectedBoolean {
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorXorFirstOperandExpectedBoolean {
                found: value.r#type().to_string(),
            }),
        }
    }

    pub fn and(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Boolean => match other {
                Self::Boolean => Ok((Self::Boolean, GeneratorExpressionOperator::And)),
                value => Err(Error::OperatorAndSecondOperandExpectedBoolean {
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorAndFirstOperandExpectedBoolean {
                found: value.r#type().to_string(),
            }),
        }
    }

    pub fn equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Self::Unit, Self::Unit) => Ok((Self::Boolean, GeneratorExpressionOperator::Equals)),
            (Self::Unit, value_2) => Err(Error::OperatorEqualsSecondOperandExpectedUnit {
                found: value_2.r#type().to_string(),
            }),
            (Self::Boolean, Self::Boolean) => {
                Ok((Self::Boolean, GeneratorExpressionOperator::Equals))
            }
            (Self::Boolean, value_2) => Err(Error::OperatorEqualsSecondOperandExpectedBoolean {
                found: value_2.r#type().to_string(),
            }),
            (Self::Integer(integer_1), Self::Integer(integer_2)) => integer_1
                .equals(integer_2)
                .map(|operator| (Self::Boolean, operator))
                .map_err(Error::Integer),
            (Self::Integer(_), value_2) => Err(Error::OperatorEqualsSecondOperandExpectedInteger {
                found: value_2.r#type().to_string(),
            }),
            (value_1, _) => Err(Error::OperatorEqualsFirstOperandExpectedPrimitiveType {
                found: value_1.r#type().to_string(),
            }),
        }
    }

    pub fn not_equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Self::Unit, Self::Unit) => Ok((Self::Boolean, GeneratorExpressionOperator::NotEquals)),
            (Self::Unit, value_2) => Err(Error::OperatorNotEqualsSecondOperandExpectedUnit {
                found: value_2.r#type().to_string(),
            }),
            (Self::Boolean, Self::Boolean) => {
                Ok((Self::Boolean, GeneratorExpressionOperator::NotEquals))
            }
            (Self::Boolean, value_2) => Err(Error::OperatorNotEqualsSecondOperandExpectedBoolean {
                found: value_2.r#type().to_string(),
            }),
            (Self::Integer(integer_1), Self::Integer(integer_2)) => integer_1
                .not_equals(integer_2)
                .map(|operator| (Self::Boolean, operator))
                .map_err(Error::Integer),
            (Self::Integer(_), value_2) => {
                Err(Error::OperatorNotEqualsSecondOperandExpectedInteger {
                    found: value_2.r#type().to_string(),
                })
            }
            (value_1, _) => Err(Error::OperatorNotEqualsFirstOperandExpectedPrimitiveType {
                found: value_1.r#type().to_string(),
            }),
        }
    }

    pub fn greater_equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .greater_equals(integer_2)
                    .map(|operator| (Self::Boolean, operator))
                    .map_err(Error::Integer),
                value => Err(Error::OperatorGreaterEqualsSecondOperandExpectedInteger {
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorGreaterEqualsFirstOperandExpectedInteger {
                found: value.r#type().to_string(),
            }),
        }
    }

    pub fn lesser_equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .lesser_equals(integer_2)
                    .map(|operator| (Self::Boolean, operator))
                    .map_err(Error::Integer),
                value => Err(Error::OperatorLesserEqualsSecondOperandExpectedInteger {
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorLesserEqualsFirstOperandExpectedInteger {
                found: value.r#type().to_string(),
            }),
        }
    }

    pub fn greater(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .greater(integer_2)
                    .map(|operator| (Self::Boolean, operator))
                    .map_err(Error::Integer),
                value => Err(Error::OperatorGreaterSecondOperandExpectedInteger {
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorGreaterFirstOperandExpectedInteger {
                found: value.r#type().to_string(),
            }),
        }
    }

    pub fn lesser(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .lesser(integer_2)
                    .map(|operator| (Self::Boolean, operator))
                    .map_err(Error::Integer),
                value => Err(Error::OperatorLesserSecondOperandExpectedInteger {
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorLesserFirstOperandExpectedInteger {
                found: value.r#type().to_string(),
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
                value => Err(Error::OperatorBitwiseOrSecondOperandExpectedInteger {
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorBitwiseOrFirstOperandExpectedInteger {
                found: value.r#type().to_string(),
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
                value => Err(Error::OperatorBitwiseXorSecondOperandExpectedInteger {
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorBitwiseXorFirstOperandExpectedInteger {
                found: value.r#type().to_string(),
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
                value => Err(Error::OperatorBitwiseAndSecondOperandExpectedInteger {
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorBitwiseAndFirstOperandExpectedInteger {
                found: value.r#type().to_string(),
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
                value => Err(
                    Error::OperatorBitwiseShiftLeftSecondOperandExpectedInteger {
                        found: value.r#type().to_string(),
                    },
                ),
            },
            value => Err(Error::OperatorBitwiseShiftLeftFirstOperandExpectedInteger {
                found: value.r#type().to_string(),
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
                value => Err(
                    Error::OperatorBitwiseShiftRightSecondOperandExpectedInteger {
                        found: value.r#type().to_string(),
                    },
                ),
            },
            value => Err(
                Error::OperatorBitwiseShiftRightFirstOperandExpectedInteger {
                    found: value.r#type().to_string(),
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
                value => Err(Error::OperatorAdditionSecondOperandExpectedInteger {
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorAdditionFirstOperandExpectedInteger {
                found: value.r#type().to_string(),
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
                value => Err(Error::OperatorSubtractionSecondOperandExpectedInteger {
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorSubtractionFirstOperandExpectedInteger {
                found: value.r#type().to_string(),
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
                value => Err(Error::OperatorMultiplicationSecondOperandExpectedInteger {
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorMultiplicationFirstOperandExpectedInteger {
                found: value.r#type().to_string(),
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
                value => Err(Error::OperatorDivisionSecondOperandExpectedInteger {
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorDivisionFirstOperandExpectedInteger {
                found: value.r#type().to_string(),
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
                value => Err(Error::OperatorRemainderSecondOperandExpectedInteger {
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorRemainderFirstOperandExpectedInteger {
                found: value.r#type().to_string(),
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
            Self::Boolean => Ok((Self::Boolean, GeneratorExpressionOperator::Not)),
            value => Err(Error::OperatorNotExpectedBoolean {
                found: value.r#type().to_string(),
            }),
        }
    }

    pub fn bitwise_not(self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer) => integer
                .bitwise_not()
                .map(|(integer, operator)| (Self::Integer(integer), operator))
                .map_err(Error::Integer),
            value => Err(Error::OperatorBitwiseNotExpectedInteger {
                found: value.r#type().to_string(),
            }),
        }
    }

    pub fn negate(self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer) => integer
                .negate()
                .map(|(integer, operator)| (Self::Integer(integer), operator))
                .map_err(Error::Integer),
            value => Err(Error::OperatorNegationExpectedInteger {
                found: value.r#type().to_string(),
            }),
        }
    }

    pub fn index_value(self, other: Self) -> Result<(Self, IndexAccess), Error> {
        match self {
            Value::Array(array) => match other {
                Value::Integer(_) => Ok(array.slice_single()),
                value => Err(Error::OperatorIndexSecondOperandExpectedIntegerOrRange {
                    found: value.to_string(),
                }),
            },
            value => Err(Error::OperatorIndexFirstOperandExpectedArray {
                found: value.to_string(),
            }),
        }
    }

    pub fn index_constant(self, other: Constant) -> Result<(Self, IndexAccess), Error> {
        match self {
            Value::Array(array) => match other {
                Constant::Integer(_) => Ok(array.slice_single()),
                Constant::Range(range) => array
                    .slice_range(range.start, range.end)
                    .map(|(value, access)| (value, access))
                    .map_err(Error::Array),
                Constant::RangeInclusive(range) => array
                    .slice_range_inclusive(range.start, range.end)
                    .map(|(value, access)| (value, access))
                    .map_err(Error::Array),
                constant => Err(Error::OperatorIndexSecondOperandExpectedIntegerOrRange {
                    found: constant.to_string(),
                }),
            },
            value => Err(Error::OperatorIndexFirstOperandExpectedArray {
                found: value.to_string(),
            }),
        }
    }

    pub fn field_tuple(self, field_index: usize) -> Result<(Self, FieldAccess), Error> {
        match self {
            Value::Tuple(tuple) => tuple.slice(field_index).map_err(Error::Tuple),
            value => Err(Error::OperatorFieldFirstOperandExpectedTuple {
                found: value.to_string(),
            }),
        }
    }

    pub fn field_structure(self, field_name: String) -> Result<(Self, FieldAccess), Error> {
        match self {
            Value::Structure(structure) => structure.slice(field_name).map_err(Error::Structure),
            value => Err(Error::OperatorFieldFirstOperandExpectedStructure {
                found: value.to_string(),
            }),
        }
    }
}

impl TryFrom<&Type> for Value {
    type Error = Error;

    fn try_from(r#type: &Type) -> Result<Self, Self::Error> {
        Ok(match r#type {
            Type::Unit => Self::Unit,
            Type::Boolean => Self::Boolean,
            Type::IntegerUnsigned { bitlength } => Self::Integer(Integer::new(false, *bitlength)),
            Type::IntegerSigned { bitlength } => Self::Integer(Integer::new(true, *bitlength)),
            Type::Field => Self::Integer(Integer::new(false, crate::BITLENGTH_FIELD)),
            Type::Array { r#type, size } => Self::Array(Array::new(*r#type.to_owned(), *size)),
            Type::Tuple { types } => Self::Tuple(Tuple::new(types.to_owned())),
            Type::Structure(structure) => Self::Structure(Structure::new(structure.to_owned())),
            Type::Enumeration(enumeration) => {
                let mut integer = Integer::new(false, enumeration.bitlength);
                integer.set_enumeration(enumeration.to_owned());
                Self::Integer(integer)
            }
            _ => panic!(crate::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),
        })
    }
}

impl TryFrom<Constant> for Value {
    type Error = Error;

    fn try_from(constant: Constant) -> Result<Self, Self::Error> {
        Self::try_from(&constant.r#type())
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unit => write!(f, "<unit> value"),
            Self::Boolean => write!(f, "<boolean> value"),
            Self::Integer(inner) => write!(f, "{}", inner),
            Self::Array(inner) => write!(f, "{}", inner),
            Self::Tuple(inner) => write!(f, "{}", inner),
            Self::Structure(inner) => write!(f, "{}", inner),
        }
    }
}
