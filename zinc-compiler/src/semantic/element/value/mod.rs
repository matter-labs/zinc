//!
//! The semantic analyzer value element.
//!

mod tests;

pub mod array;
pub mod boolean;
pub mod contract;
pub mod error;
pub mod integer;
pub mod structure;
pub mod tuple;
pub mod unit;

use std::fmt;

use crate::generator::expression::operator::Operator as GeneratorExpressionOperator;
use crate::lexical::token::location::Location;
use crate::semantic::casting::Caster;
use crate::semantic::element::access::dot::stack_field::StackField as StackFieldAccess;
use crate::semantic::element::access::index::Index as IndexAccess;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::place::Place;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::tuple_index::TupleIndex;
use crate::syntax::tree::identifier::Identifier;

use self::array::Array;
use self::boolean::Boolean;
use self::contract::Contract;
use self::error::Error;
use self::integer::Integer;
use self::structure::Structure;
use self::tuple::Tuple;
use self::unit::Unit;

///
/// Value are parts of a non-constant expression.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Unit(Unit),
    Boolean(Boolean),
    Integer(Integer),
    Array(Array),
    Tuple(Tuple),
    Structure(Structure),
    Contract(Contract),
}

impl Value {
    pub fn r#type(&self) -> Type {
        match self {
            Self::Unit(inner) => inner.r#type(),
            Self::Boolean(inner) => inner.r#type(),
            Self::Integer(inner) => inner.r#type(),
            Self::Array(inner) => inner.r#type(),
            Self::Tuple(inner) => inner.r#type(),
            Self::Structure(inner) => inner.r#type(),
            Self::Contract(inner) => inner.r#type(),
        }
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Unit(_), Self::Unit(_)) => true,
            (Self::Boolean(_), Self::Boolean(_)) => true,
            (Self::Integer(value_1), Self::Integer(value_2)) => {
                value_1.has_the_same_type_as(value_2)
            }
            (Self::Array(value_1), Self::Array(value_2)) => value_1.has_the_same_type_as(value_2),
            (Self::Tuple(value_1), Self::Tuple(value_2)) => value_1.has_the_same_type_as(value_2),
            (Self::Structure(value_1), Self::Structure(value_2)) => {
                value_1.has_the_same_type_as(value_2)
            }
            (Self::Contract(value_1), Self::Contract(value_2)) => {
                value_1.has_the_same_type_as(value_2)
            }
            _ => false,
        }
    }

    pub fn or(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Boolean(value_1) => match other {
                Self::Boolean(_) => Ok((Self::Boolean(value_1), GeneratorExpressionOperator::Or)),
                value => Err(Error::OperatorOrSecondOperandExpectedBoolean {
                    location: value
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorOrFirstOperandExpectedBoolean {
                location: value
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                found: value.r#type().to_string(),
            }),
        }
    }

    pub fn xor(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Boolean(value_1) => match other {
                Self::Boolean(_) => Ok((Self::Boolean(value_1), GeneratorExpressionOperator::Xor)),
                value => Err(Error::OperatorXorSecondOperandExpectedBoolean {
                    location: value
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorXorFirstOperandExpectedBoolean {
                location: value
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                found: value.r#type().to_string(),
            }),
        }
    }

    pub fn and(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Boolean(value_1) => match other {
                Self::Boolean(_) => Ok((Self::Boolean(value_1), GeneratorExpressionOperator::And)),
                value => Err(Error::OperatorAndSecondOperandExpectedBoolean {
                    location: value
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorAndFirstOperandExpectedBoolean {
                location: value
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                found: value.r#type().to_string(),
            }),
        }
    }

    pub fn equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Self::Unit(value_1), Self::Unit(_)) => Ok((
                Self::Boolean(Boolean::new(value_1.location)),
                GeneratorExpressionOperator::Equals,
            )),
            (Self::Unit(_), value_2) => Err(Error::OperatorEqualsSecondOperandExpectedUnit {
                location: value_2
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                found: value_2.r#type().to_string(),
            }),
            (Self::Boolean(value_1), Self::Boolean(_)) => {
                Ok((Self::Boolean(value_1), GeneratorExpressionOperator::Equals))
            }
            (Self::Boolean(_), value_2) => Err(Error::OperatorEqualsSecondOperandExpectedBoolean {
                location: value_2
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                found: value_2.r#type().to_string(),
            }),
            (Self::Integer(integer_1), Self::Integer(integer_2)) => {
                let location = integer_1.location;

                integer_1
                    .equals(integer_2)
                    .map(|operator| (Self::Boolean(Boolean::new(location)), operator))
                    .map_err(Error::Integer)
            }
            (Self::Integer(_), value_2) => Err(Error::OperatorEqualsSecondOperandExpectedInteger {
                location: value_2
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                found: value_2.r#type().to_string(),
            }),
            (value_1, _) => Err(Error::OperatorEqualsFirstOperandExpectedPrimitiveType {
                location: value_1
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                found: value_1.r#type().to_string(),
            }),
        }
    }

    pub fn not_equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Self::Unit(value_1), Self::Unit(_)) => Ok((
                Self::Boolean(Boolean::new(value_1.location)),
                GeneratorExpressionOperator::NotEquals,
            )),
            (Self::Unit(_), value_2) => Err(Error::OperatorNotEqualsSecondOperandExpectedUnit {
                location: value_2
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                found: value_2.r#type().to_string(),
            }),
            (Self::Boolean(value_1), Self::Boolean(_)) => Ok((
                Self::Boolean(value_1),
                GeneratorExpressionOperator::NotEquals,
            )),
            (Self::Boolean(_), value_2) => {
                Err(Error::OperatorNotEqualsSecondOperandExpectedBoolean {
                    location: value_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: value_2.r#type().to_string(),
                })
            }
            (Self::Integer(integer_1), Self::Integer(integer_2)) => {
                let location = integer_1.location;

                integer_1
                    .not_equals(integer_2)
                    .map(|operator| (Self::Boolean(Boolean::new(location)), operator))
                    .map_err(Error::Integer)
            }
            (Self::Integer(_), value_2) => {
                Err(Error::OperatorNotEqualsSecondOperandExpectedInteger {
                    location: value_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: value_2.r#type().to_string(),
                })
            }
            (value_1, _) => Err(Error::OperatorNotEqualsFirstOperandExpectedPrimitiveType {
                location: value_1
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                found: value_1.r#type().to_string(),
            }),
        }
    }

    pub fn greater_equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => {
                    let location = integer_1.location;

                    integer_1
                        .greater_equals(integer_2)
                        .map(|operator| (Self::Boolean(Boolean::new(location)), operator))
                        .map_err(Error::Integer)
                }
                value => Err(Error::OperatorGreaterEqualsSecondOperandExpectedInteger {
                    location: value
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorGreaterEqualsFirstOperandExpectedInteger {
                location: value
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                found: value.r#type().to_string(),
            }),
        }
    }

    pub fn lesser_equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => {
                    let location = integer_1.location;

                    integer_1
                        .lesser_equals(integer_2)
                        .map(|operator| (Self::Boolean(Boolean::new(location)), operator))
                        .map_err(Error::Integer)
                }
                value => Err(Error::OperatorLesserEqualsSecondOperandExpectedInteger {
                    location: value
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorLesserEqualsFirstOperandExpectedInteger {
                location: value
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                found: value.r#type().to_string(),
            }),
        }
    }

    pub fn greater(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => {
                    let location = integer_1.location;

                    integer_1
                        .greater(integer_2)
                        .map(|operator| (Self::Boolean(Boolean::new(location)), operator))
                        .map_err(Error::Integer)
                }
                value => Err(Error::OperatorGreaterSecondOperandExpectedInteger {
                    location: value
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorGreaterFirstOperandExpectedInteger {
                location: value
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                found: value.r#type().to_string(),
            }),
        }
    }

    pub fn lesser(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => {
                    let location = integer_1.location;

                    integer_1
                        .lesser(integer_2)
                        .map(|operator| (Self::Boolean(Boolean::new(location)), operator))
                        .map_err(Error::Integer)
                }
                value => Err(Error::OperatorLesserSecondOperandExpectedInteger {
                    location: value
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorLesserFirstOperandExpectedInteger {
                location: value
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                    location: value
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorBitwiseOrFirstOperandExpectedInteger {
                location: value
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                    location: value
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorBitwiseXorFirstOperandExpectedInteger {
                location: value
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                    location: value
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorBitwiseAndFirstOperandExpectedInteger {
                location: value
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                        location: value
                            .location()
                            .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                        found: value.r#type().to_string(),
                    },
                ),
            },
            value => Err(Error::OperatorBitwiseShiftLeftFirstOperandExpectedInteger {
                location: value
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                        location: value
                            .location()
                            .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                        found: value.r#type().to_string(),
                    },
                ),
            },
            value => Err(
                Error::OperatorBitwiseShiftRightFirstOperandExpectedInteger {
                    location: value
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                    location: value
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorAdditionFirstOperandExpectedInteger {
                location: value
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                    location: value
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorSubtractionFirstOperandExpectedInteger {
                location: value
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                    location: value
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorMultiplicationFirstOperandExpectedInteger {
                location: value
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                    location: value
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorDivisionFirstOperandExpectedInteger {
                location: value
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                    location: value
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorRemainderFirstOperandExpectedInteger {
                location: value
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                found: value.r#type().to_string(),
            }),
        }
    }

    pub fn cast(self, to: Type) -> Result<(Self, Option<GeneratorExpressionOperator>), Error> {
        let from = self.r#type();
        Caster::cast(&from, &to).map_err(|error| Error::Casting {
            location: self.location().expect(crate::panic::LOCATION_ALWAYS_EXISTS),
            inner: error,
            reference: to.location(),
        })?;

        let (is_signed, bitlength) = match to {
            Type::IntegerUnsigned { bitlength, .. } => (false, bitlength),
            Type::IntegerSigned { bitlength, .. } => (true, bitlength),
            Type::Field(_) => (false, crate::BITLENGTH_FIELD),
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
            Self::Boolean(inner) => Ok((Self::Boolean(inner), GeneratorExpressionOperator::Not)),
            value => Err(Error::OperatorNotExpectedBoolean {
                location: value
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                location: value
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                location: value
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                found: value.r#type().to_string(),
            }),
        }
    }

    pub fn index_value(self, other: Self) -> Result<(Self, IndexAccess), Error> {
        match self {
            Value::Array(array) => match other {
                Value::Integer(_) => Ok(array.slice_single()),
                value => Err(Error::OperatorIndexSecondOperandExpectedIntegerOrRange {
                    location: value
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: value.to_string(),
                }),
            },
            value => Err(Error::OperatorIndexFirstOperandExpectedArray {
                location: value
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                found: value.to_string(),
            }),
        }
    }

    pub fn index_constant(self, other: Constant) -> Result<(Self, IndexAccess), Error> {
        match self {
            Value::Array(array) => match other {
                Constant::Integer(_) => Ok(array.slice_single()),
                Constant::Range(range) => array
                    .slice_range(range)
                    .map(|(value, access)| (value, access))
                    .map_err(Error::Array),
                Constant::RangeInclusive(range) => array
                    .slice_range_inclusive(range)
                    .map(|(value, access)| (value, access))
                    .map_err(Error::Array),
                constant => Err(Error::OperatorIndexSecondOperandExpectedIntegerOrRange {
                    location: constant.location(),
                    found: constant.to_string(),
                }),
            },
            value => Err(Error::OperatorIndexFirstOperandExpectedArray {
                location: value
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                found: value.to_string(),
            }),
        }
    }

    pub fn tuple_field(self, tuple_index: TupleIndex) -> Result<(Self, StackFieldAccess), Error> {
        match self {
            Value::Tuple(tuple) => tuple.slice(tuple_index).map_err(Error::Tuple),
            value => Err(Error::OperatorDotFirstOperandExpectedTuple {
                location: value
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                found: value.to_string(),
            }),
        }
    }

    pub fn structure_field(
        self,
        identifier: Identifier,
    ) -> Result<(Self, StackFieldAccess), Error> {
        match self {
            Value::Structure(structure) => structure.slice(identifier).map_err(Error::Structure),
            value => Err(Error::OperatorDotFirstOperandExpectedStructure {
                location: value
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                found: value.to_string(),
            }),
        }
    }

    pub fn try_from_place(place: &Place) -> Result<Self, Error> {
        let location = place.identifier.location;

        Self::try_from_type(&place.r#type, Some(location))
    }

    pub fn try_from_type(r#type: &Type, location: Option<Location>) -> Result<Self, Error> {
        Ok(match r#type {
            Type::Unit(_) => Self::Unit(Unit::new(location.or_else(|| r#type.location()))),
            Type::Boolean(_) => Self::Boolean(Boolean::new(location.or_else(|| r#type.location()))),
            Type::IntegerUnsigned { bitlength, .. } => Self::Integer(Integer::new(
                location.or_else(|| r#type.location()),
                false,
                *bitlength,
            )),
            Type::IntegerSigned { bitlength, .. } => Self::Integer(Integer::new(
                location.or_else(|| r#type.location()),
                true,
                *bitlength,
            )),
            Type::Field(_) => Self::Integer(Integer::new(
                location.or_else(|| r#type.location()),
                false,
                crate::BITLENGTH_FIELD,
            )),
            Type::Array(inner) => Self::Array(Array::new_with_values(
                location.or_else(|| inner.location.to_owned()),
                *inner.r#type.to_owned(),
                inner.size,
            )),
            Type::Tuple(inner) => Self::Tuple(Tuple::new_with_values(
                location.or_else(|| inner.location),
                inner.types.to_owned(),
            )),
            Type::Structure(inner) => Self::Structure(Structure::new_with_type(
                location.or_else(|| inner.location),
                inner.to_owned(),
            )),
            Type::Enumeration(inner) => {
                let mut integer =
                    Integer::new(location.or_else(|| inner.location), false, inner.bitlength);
                integer.set_enumeration(inner.to_owned());
                Self::Integer(integer)
            }
            Type::Contract(inner) => Self::Contract(Contract::new(
                location.or_else(|| inner.location),
                inner.to_owned(),
            )),
            _ => panic!(crate::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),
        })
    }

    pub fn try_from_constant(constant: Constant) -> Result<Self, Error> {
        Self::try_from_type(&constant.r#type(), Some(constant.location()))
    }

    pub fn location(&self) -> Option<Location> {
        match self {
            Self::Unit(inner) => inner.location,
            Self::Boolean(inner) => inner.location,
            Self::Integer(inner) => inner.location,
            Self::Array(inner) => inner.location,
            Self::Tuple(inner) => inner.location,
            Self::Structure(inner) => inner.location,
            Self::Contract(inner) => inner.location,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unit(inner) => write!(f, "unit {}", inner),
            Self::Boolean(inner) => write!(f, "boolean {}", inner),
            Self::Integer(inner) => write!(f, "integer {}", inner),
            Self::Array(inner) => write!(f, "array {}", inner),
            Self::Tuple(inner) => write!(f, "tuple {}", inner),
            Self::Structure(inner) => write!(f, "structure {}", inner),
            Self::Contract(inner) => write!(f, "contract {}", inner),
        }
    }
}
