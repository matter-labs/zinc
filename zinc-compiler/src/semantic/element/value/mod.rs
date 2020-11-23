//!
//! The semantic analyzer value element.
//!

#[cfg(test)]
mod tests;

pub mod array;
pub mod boolean;
pub mod contract;
pub mod integer;
pub mod structure;
pub mod tuple;
pub mod unit;

use std::fmt;
use std::ops::Add;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::BitXor;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Rem;
use std::ops::Shl;
use std::ops::Shr;
use std::ops::Sub;

use zinc_lexical::Location;
use zinc_syntax::Identifier;

use crate::generator::expression::operator::Operator as GeneratorExpressionOperator;
use crate::semantic::casting::Caster;
use crate::semantic::element::access::dot::stack_field::StackField as StackFieldAccess;
use crate::semantic::element::access::dot::Dot as DotAccessVariant;
use crate::semantic::element::access::index::Index as IndexAccess;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::place::Place;
use crate::semantic::element::r#type::i_typed::ITyped;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::tuple_index::TupleIndex;
use crate::semantic::error::Error;

use self::array::Array;
use self::boolean::Boolean;
use self::contract::Contract;
use self::integer::Integer;
use self::structure::Structure;
use self::tuple::Tuple;
use self::unit::Unit;

///
/// Value are parts of a non-constant expression.
///
/// If a constant value of an expression cannot be known, it coersed to a value, that is, to a
/// value, which will be only known at runtime.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// The unit `()` type value.
    Unit(Unit),
    /// The boolean type value.
    Boolean(Boolean),
    /// The integer type value.
    Integer(Integer),
    /// The array type value.
    Array(Array),
    /// The tuple type value.
    Tuple(Tuple),
    /// The structure type value.
    Structure(Structure),
    /// The contract type value.
    Contract(Contract),
}

impl Value {
    ///
    /// Executes the `||` logical OR operator.
    ///
    pub fn or(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Boolean(value_1) => match other {
                Self::Boolean(_) => Ok((Self::Boolean(value_1), GeneratorExpressionOperator::Or)),
                value => Err(Error::OperatorOrSecondOperandExpectedBoolean {
                    location: value
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorOrFirstOperandExpectedBoolean {
                location: value
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value.r#type().to_string(),
            }),
        }
    }

    ///
    /// Executes the `^^` logical XOR operator.
    ///
    pub fn xor(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Boolean(value_1) => match other {
                Self::Boolean(_) => Ok((Self::Boolean(value_1), GeneratorExpressionOperator::Xor)),
                value => Err(Error::OperatorXorSecondOperandExpectedBoolean {
                    location: value
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorXorFirstOperandExpectedBoolean {
                location: value
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value.r#type().to_string(),
            }),
        }
    }

    ///
    /// Executes the `&&` logical AND operator.
    ///
    pub fn and(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Boolean(value_1) => match other {
                Self::Boolean(_) => Ok((Self::Boolean(value_1), GeneratorExpressionOperator::And)),
                value => Err(Error::OperatorAndSecondOperandExpectedBoolean {
                    location: value
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorAndFirstOperandExpectedBoolean {
                location: value
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value.r#type().to_string(),
            }),
        }
    }

    ///
    /// Executes the `==` equals comparison operator.
    ///
    pub fn equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Self::Unit(value_1), Self::Unit(_)) => Ok((
                Self::Boolean(Boolean::new(value_1.location)),
                GeneratorExpressionOperator::equals(),
            )),
            (Self::Unit(_), value_2) => Err(Error::OperatorEqualsSecondOperandExpectedUnit {
                location: value_2
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value_2.r#type().to_string(),
            }),
            (Self::Boolean(value_1), Self::Boolean(_)) => Ok((
                Self::Boolean(value_1),
                GeneratorExpressionOperator::equals(),
            )),
            (Self::Boolean(_), value_2) => Err(Error::OperatorEqualsSecondOperandExpectedBoolean {
                location: value_2
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value_2.r#type().to_string(),
            }),
            (Self::Integer(integer_1), Self::Integer(integer_2)) => {
                let location = integer_1.location;

                integer_1
                    .equals(integer_2)
                    .map(|operator| (Self::Boolean(Boolean::new(location)), operator))
            }
            (Self::Integer(_), value_2) => Err(Error::OperatorEqualsSecondOperandExpectedInteger {
                location: value_2
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value_2.r#type().to_string(),
            }),
            (value_1, _) => Err(Error::OperatorEqualsFirstOperandExpectedPrimitiveType {
                location: value_1
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value_1.r#type().to_string(),
            }),
        }
    }

    ///
    /// Executes the `!=` not-equals comparison operator.
    ///
    pub fn not_equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Self::Unit(value_1), Self::Unit(_)) => Ok((
                Self::Boolean(Boolean::new(value_1.location)),
                GeneratorExpressionOperator::not_equals(),
            )),
            (Self::Unit(_), value_2) => Err(Error::OperatorNotEqualsSecondOperandExpectedUnit {
                location: value_2
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value_2.r#type().to_string(),
            }),
            (Self::Boolean(value_1), Self::Boolean(_)) => Ok((
                Self::Boolean(value_1),
                GeneratorExpressionOperator::not_equals(),
            )),
            (Self::Boolean(_), value_2) => {
                Err(Error::OperatorNotEqualsSecondOperandExpectedBoolean {
                    location: value_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: value_2.r#type().to_string(),
                })
            }
            (Self::Integer(integer_1), Self::Integer(integer_2)) => {
                let location = integer_1.location;

                integer_1
                    .not_equals(integer_2)
                    .map(|operator| (Self::Boolean(Boolean::new(location)), operator))
            }
            (Self::Integer(_), value_2) => {
                Err(Error::OperatorNotEqualsSecondOperandExpectedInteger {
                    location: value_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: value_2.r#type().to_string(),
                })
            }
            (value_1, _) => Err(Error::OperatorNotEqualsFirstOperandExpectedPrimitiveType {
                location: value_1
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value_1.r#type().to_string(),
            }),
        }
    }

    ///
    /// Executes the `>=` greater-equals comparison operator.
    ///
    pub fn greater_equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => {
                    let location = integer_1.location;

                    integer_1
                        .greater_equals(integer_2)
                        .map(|operator| (Self::Boolean(Boolean::new(location)), operator))
                }
                value => Err(Error::OperatorGreaterEqualsSecondOperandExpectedInteger {
                    location: value
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorGreaterEqualsFirstOperandExpectedInteger {
                location: value
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value.r#type().to_string(),
            }),
        }
    }

    ///
    /// Executes the `<=` lesser-equals comparison operator.
    ///
    pub fn lesser_equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => {
                    let location = integer_1.location;

                    integer_1
                        .lesser_equals(integer_2)
                        .map(|operator| (Self::Boolean(Boolean::new(location)), operator))
                }
                value => Err(Error::OperatorLesserEqualsSecondOperandExpectedInteger {
                    location: value
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorLesserEqualsFirstOperandExpectedInteger {
                location: value
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value.r#type().to_string(),
            }),
        }
    }

    ///
    /// Executes the `>` greater comparison operator.
    ///
    pub fn greater(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => {
                    let location = integer_1.location;

                    integer_1
                        .greater(integer_2)
                        .map(|operator| (Self::Boolean(Boolean::new(location)), operator))
                }
                value => Err(Error::OperatorGreaterSecondOperandExpectedInteger {
                    location: value
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorGreaterFirstOperandExpectedInteger {
                location: value
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value.r#type().to_string(),
            }),
        }
    }

    ///
    /// Executes the `<` lesser comparison operator.
    ///
    pub fn lesser(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => {
                    let location = integer_1.location;

                    integer_1
                        .lesser(integer_2)
                        .map(|operator| (Self::Boolean(Boolean::new(location)), operator))
                }
                value => Err(Error::OperatorLesserSecondOperandExpectedInteger {
                    location: value
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorLesserFirstOperandExpectedInteger {
                location: value
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value.r#type().to_string(),
            }),
        }
    }
}

impl BitOr for Value {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn bitor(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .bitor(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator)),
                value => Err(Error::OperatorBitwiseOrSecondOperandExpectedInteger {
                    location: value
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorBitwiseOrFirstOperandExpectedInteger {
                location: value
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value.r#type().to_string(),
            }),
        }
    }
}

impl BitXor for Value {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn bitxor(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .bitxor(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator)),
                value => Err(Error::OperatorBitwiseXorSecondOperandExpectedInteger {
                    location: value
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorBitwiseXorFirstOperandExpectedInteger {
                location: value
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value.r#type().to_string(),
            }),
        }
    }
}

impl BitAnd for Value {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn bitand(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .bitand(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator)),
                value => Err(Error::OperatorBitwiseAndSecondOperandExpectedInteger {
                    location: value
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorBitwiseAndFirstOperandExpectedInteger {
                location: value
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value.r#type().to_string(),
            }),
        }
    }
}

impl Shl<Self> for Value {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn shl(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .shl(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator)),
                value => Err(
                    Error::OperatorBitwiseShiftLeftSecondOperandExpectedInteger {
                        location: value
                            .location()
                            .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                        found: value.r#type().to_string(),
                    },
                ),
            },
            value => Err(Error::OperatorBitwiseShiftLeftFirstOperandExpectedInteger {
                location: value
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value.r#type().to_string(),
            }),
        }
    }
}

impl Shr<Self> for Value {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn shr(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .shr(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator)),
                value => Err(
                    Error::OperatorBitwiseShiftRightSecondOperandExpectedInteger {
                        location: value
                            .location()
                            .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                        found: value.r#type().to_string(),
                    },
                ),
            },
            value => Err(
                Error::OperatorBitwiseShiftRightFirstOperandExpectedInteger {
                    location: value
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                },
            ),
        }
    }
}

impl Add for Value {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn add(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .add(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator)),
                value => Err(Error::OperatorAdditionSecondOperandExpectedInteger {
                    location: value
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorAdditionFirstOperandExpectedInteger {
                location: value
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value.r#type().to_string(),
            }),
        }
    }
}

impl Sub for Value {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn sub(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .sub(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator)),
                value => Err(Error::OperatorSubtractionSecondOperandExpectedInteger {
                    location: value
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorSubtractionFirstOperandExpectedInteger {
                location: value
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value.r#type().to_string(),
            }),
        }
    }
}

impl Mul for Value {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn mul(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .mul(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator)),
                value => Err(Error::OperatorMultiplicationSecondOperandExpectedInteger {
                    location: value
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorMultiplicationFirstOperandExpectedInteger {
                location: value
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value.r#type().to_string(),
            }),
        }
    }
}

impl Div for Value {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn div(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .div(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator)),
                value => Err(Error::OperatorDivisionSecondOperandExpectedInteger {
                    location: value
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorDivisionFirstOperandExpectedInteger {
                location: value
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value.r#type().to_string(),
            }),
        }
    }
}

impl Rem for Value {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn rem(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .rem(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator)),
                value => Err(Error::OperatorRemainderSecondOperandExpectedInteger {
                    location: value
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: value.r#type().to_string(),
                }),
            },
            value => Err(Error::OperatorRemainderFirstOperandExpectedInteger {
                location: value
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value.r#type().to_string(),
            }),
        }
    }
}

impl Value {
    ///
    /// Executes the `as` casting operator.
    ///
    pub fn cast(self, to: Type) -> Result<(Self, Option<GeneratorExpressionOperator>), Error> {
        let from = self.r#type();
        Caster::cast(&from, &to).map_err(|error| Error::OperatorCastingTypesMismatch {
            location: self
                .location()
                .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
            inner: error,
            reference: to.location().expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
        })?;

        let (is_signed, bitlength) = match to {
            Type::IntegerUnsigned { bitlength, .. } => (false, bitlength),
            Type::IntegerSigned { bitlength, .. } => (true, bitlength),
            Type::Field(_) => (false, zinc_const::bitlength::FIELD),
            _ => return Ok((self, None)),
        };

        Ok(match self {
            Self::Integer(integer) => integer
                .cast(is_signed, bitlength)
                .map(|(integer, operator)| (Self::Integer(integer), operator))?,
            operand => (operand, None),
        })
    }

    ///
    /// Executes the `!` logical NOT operator.
    ///
    pub fn not(self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Boolean(inner) => Ok((Self::Boolean(inner), GeneratorExpressionOperator::Not)),
            value => Err(Error::OperatorNotExpectedBoolean {
                location: value
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value.r#type().to_string(),
            }),
        }
    }

    ///
    /// Executes the `~` bitwise NOT operator.
    ///
    pub fn bitwise_not(self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer) => integer
                .bitwise_not()
                .map(|(integer, operator)| (Self::Integer(integer), operator)),
            value => Err(Error::OperatorBitwiseNotExpectedInteger {
                location: value
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value.r#type().to_string(),
            }),
        }
    }
}

impl Neg for Value {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn neg(self) -> Self::Output {
        match self {
            Self::Integer(integer) => integer
                .neg()
                .map(|(integer, operator)| (Self::Integer(integer), operator)),
            value => Err(Error::OperatorNegationExpectedInteger {
                location: value
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value.r#type().to_string(),
            }),
        }
    }
}

impl Value {
    ///
    /// Executes the `[]` array index operator with a non-constant value.
    ///
    pub fn index_value(self, other: Self) -> Result<(Self, IndexAccess), Error> {
        match self {
            Value::Array(array) => match other {
                Value::Integer(_) => Ok(array.slice_single()),
                value => Err(Error::OperatorIndexSecondOperandExpectedIntegerOrRange {
                    location: value
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: value.to_string(),
                }),
            },
            value => Err(Error::OperatorIndexFirstOperandExpectedArray {
                location: value
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value.to_string(),
            }),
        }
    }

    ///
    /// Executes the `[]` array index operator with a constant value.
    ///
    pub fn index_constant(self, other: Constant) -> Result<(Self, IndexAccess), Error> {
        match self {
            Value::Array(array) => match other {
                Constant::Integer(_) => Ok(array.slice_single()),
                Constant::Range(range) => array
                    .slice_range(range)
                    .map(|(value, access)| (value, access)),
                Constant::RangeInclusive(range) => array
                    .slice_range_inclusive(range)
                    .map(|(value, access)| (value, access)),
                constant => Err(Error::OperatorIndexSecondOperandExpectedIntegerOrRange {
                    location: constant.location(),
                    found: constant.to_string(),
                }),
            },
            value => Err(Error::OperatorIndexFirstOperandExpectedArray {
                location: value
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value.to_string(),
            }),
        }
    }

    ///
    /// Executes the `.` dot field access operator for a tuple.
    ///
    pub fn tuple_field(self, tuple_index: TupleIndex) -> Result<(Self, StackFieldAccess), Error> {
        match self {
            Value::Tuple(tuple) => tuple.slice(tuple_index),
            value => Err(Error::OperatorDotFirstOperandExpectedTuple {
                location: value
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value.to_string(),
            }),
        }
    }

    ///
    /// Executes the `.` dot field access operator for a structure.
    ///
    pub fn structure_field(
        self,
        identifier: Identifier,
    ) -> Result<(Self, DotAccessVariant), Error> {
        match self {
            Value::Structure(structure) => structure
                .slice(identifier)
                .map(|(value, access)| (value, DotAccessVariant::StackField(access))),
            Value::Contract(contract) => contract
                .slice(identifier)
                .map(|(value, access)| (value, DotAccessVariant::ContractField(access))),
            value => Err(Error::OperatorDotFirstOperandExpectedInstance {
                location: value
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value.to_string(),
            }),
        }
    }

    ///
    /// Tries to create a value from the memory place's type, passing it to the
    /// `try_from_type` function.
    ///
    pub fn try_from_place(place: &Place) -> Result<Self, Error> {
        let location = place.identifier.location;

        Self::try_from_type(&place.r#type, false, Some(location))
    }

    ///
    /// Tries to create a value from the `r#type`.
    ///
    /// Values can be created from any type, except functions and ranges.
    ///
    pub fn try_from_type(
        r#type: &Type,
        is_literal: bool,
        location: Option<Location>,
    ) -> Result<Self, Error> {
        Ok(match r#type {
            Type::Unit(_) => Self::Unit(Unit::new(location.or_else(|| r#type.location()))),
            Type::Boolean(_) => Self::Boolean(Boolean::new(location.or_else(|| r#type.location()))),
            Type::IntegerUnsigned { bitlength, .. } => Self::Integer(Integer::new(
                location.or_else(|| r#type.location()),
                false,
                *bitlength,
                is_literal,
            )),
            Type::IntegerSigned { bitlength, .. } => Self::Integer(Integer::new(
                location.or_else(|| r#type.location()),
                true,
                *bitlength,
                is_literal,
            )),
            Type::Field(_) => Self::Integer(Integer::new(
                location.or_else(|| r#type.location()),
                false,
                zinc_const::bitlength::FIELD,
                is_literal,
            )),
            Type::Array(inner) => Self::Array(Array::new_with_values(
                location.or(inner.location),
                *inner.r#type.to_owned(),
                inner.size,
            )),
            Type::Tuple(inner) => Self::Tuple(Tuple::new_with_values(
                location.or(inner.location),
                inner.types.to_owned(),
            )),
            Type::Structure(inner) => Self::Structure(Structure::new_with_type(
                location.or(inner.location),
                inner.to_owned(),
            )),
            Type::Enumeration(inner) => {
                let mut integer =
                    Integer::new(location.or(inner.location), false, inner.bitlength, false);
                integer.set_enumeration(inner.to_owned());
                Self::Integer(integer)
            }
            Type::Contract(inner) => Self::Contract(Contract::new_with_type(
                Some(location.unwrap_or(inner.location)),
                inner.to_owned(),
            )),
            _ => panic!(zinc_const::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),
        })
    }

    ///
    /// Tries to create a value from the `constant`, passing its type to the
    /// `try_from_type` function.
    ///
    pub fn try_from_constant(constant: Constant) -> Result<Self, Error> {
        Self::try_from_type(
            &constant.r#type(),
            constant.is_literal(),
            Some(constant.location()),
        )
    }

    ///
    /// Returns the constant location in the code.
    ///
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

impl ITyped for Value {
    fn r#type(&self) -> Type {
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

    fn has_the_same_type_as(&self, other: &Self) -> bool {
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
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
