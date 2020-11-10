//!
//! The semantic analyzer constant element.
//!

#[cfg(test)]
mod tests;

pub mod array;
pub mod boolean;
pub mod integer;
pub mod range;
pub mod range_inclusive;
pub mod string;
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
use crate::semantic::element::access::index::Index as IndexAccess;
use crate::semantic::element::r#type::i_typed::ITyped;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::tuple_index::TupleIndex;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error;

use self::array::Array;
use self::boolean::Boolean;
use self::integer::Integer;
use self::range::Range;
use self::range_inclusive::RangeInclusive;
use self::string::String;
use self::structure::Structure;
use self::tuple::Tuple;
use self::unit::Unit;

///
/// Constants are parts of a constant expression.
///
/// The compiler preserves the constant values as long as it can.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    /// The unit `()` type constant.
    Unit(Unit),
    /// The boolean type constant.
    Boolean(Boolean),
    /// The integer type value.
    Integer(Integer),
    /// The range type value.
    Range(Range),
    /// The inclusive range type value.
    RangeInclusive(RangeInclusive),
    /// The string type value.
    String(String),
    /// The array type value.
    Array(Array),
    /// The tuple type value.
    Tuple(Tuple),
    /// The structure type value.
    Structure(Structure),
}

impl Constant {
    ///
    /// Executes the `..=` range inclusive operator.
    ///
    pub fn range_inclusive(self, other: Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .range_inclusive(integer_2)
                    .map(Self::RangeInclusive),
                constant => Err(Error::OperatorRangeInclusiveSecondOperandExpectedInteger {
                    location: constant.location(),
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorRangeInclusiveFirstOperandExpectedInteger {
                location: constant.location(),
                found: constant.to_string(),
            }),
        }
    }

    ///
    /// Executes the `..` range operator.
    ///
    pub fn range(self, other: Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1.range(integer_2).map(Self::Range),
                constant => Err(Error::OperatorRangeSecondOperandExpectedInteger {
                    location: constant.location(),
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorRangeFirstOperandExpectedInteger {
                location: constant.location(),
                found: constant.to_string(),
            }),
        }
    }

    ///
    /// Executes the `||` logical OR operator.
    ///
    pub fn or(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Boolean(constant_1) => match other {
                Self::Boolean(constant_2) => Ok((
                    Self::Boolean(constant_1.or(constant_2)),
                    GeneratorExpressionOperator::Or,
                )),
                constant => Err(Error::OperatorOrSecondOperandExpectedBoolean {
                    location: constant.location(),
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorOrFirstOperandExpectedBoolean {
                location: constant.location(),
                found: constant.to_string(),
            }),
        }
    }

    ///
    /// Executes the `^^` logical XOR operator.
    ///
    pub fn xor(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Boolean(constant_1) => match other {
                Self::Boolean(constant_2) => Ok((
                    Self::Boolean(constant_1.xor(constant_2)),
                    GeneratorExpressionOperator::Xor,
                )),
                constant => Err(Error::OperatorXorSecondOperandExpectedBoolean {
                    location: constant.location(),
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorXorFirstOperandExpectedBoolean {
                location: constant.location(),
                found: constant.to_string(),
            }),
        }
    }

    ///
    /// Executes the `&&` logical AND operator.
    ///
    pub fn and(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Boolean(constant_1) => match other {
                Self::Boolean(constant_2) => Ok((
                    Self::Boolean(constant_1.and(constant_2)),
                    GeneratorExpressionOperator::And,
                )),
                constant => Err(Error::OperatorAndSecondOperandExpectedBoolean {
                    location: constant.location(),
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorAndFirstOperandExpectedBoolean {
                location: constant.location(),
                found: constant.to_string(),
            }),
        }
    }

    ///
    /// Executes the `==` equals comparison operator.
    ///
    pub fn equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Self::Unit(constant_1), Self::Unit(_)) => Ok((
                Self::Boolean(Boolean::new(constant_1.location, true)),
                GeneratorExpressionOperator::equals(),
            )),
            (Self::Unit(_), constant_2) => Err(Error::OperatorEqualsSecondOperandExpectedUnit {
                location: constant_2.location(),
                found: constant_2.to_string(),
            }),
            (Self::Boolean(constant_1), Self::Boolean(constant_2)) => Ok((
                Self::Boolean(constant_1.equals(constant_2)),
                GeneratorExpressionOperator::equals(),
            )),
            (Self::Boolean(_), constant_2) => {
                Err(Error::OperatorEqualsSecondOperandExpectedBoolean {
                    location: constant_2.location(),
                    found: constant_2.to_string(),
                })
            }
            (Self::Integer(constant_1), Self::Integer(constant_2)) => constant_1
                .equals(constant_2)
                .map(|(boolean, operator)| (Self::Boolean(boolean), operator)),
            (Self::Integer(_), constant_2) => {
                Err(Error::OperatorEqualsSecondOperandExpectedInteger {
                    location: constant_2.location(),
                    found: constant_2.to_string(),
                })
            }
            (constant_1, _) => Err(Error::OperatorEqualsFirstOperandExpectedPrimitiveType {
                location: constant_1.location(),
                found: constant_1.to_string(),
            }),
        }
    }

    ///
    /// Executes the `!=` not-equals comparison operator.
    ///
    pub fn not_equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Self::Unit(constant_1), Self::Unit(_)) => Ok((
                Self::Boolean(Boolean::new(constant_1.location, false)),
                GeneratorExpressionOperator::not_equals(),
            )),
            (Self::Unit(_), constant_2) => Err(Error::OperatorNotEqualsSecondOperandExpectedUnit {
                location: constant_2.location(),
                found: constant_2.to_string(),
            }),
            (Self::Boolean(constant_1), Self::Boolean(constant_2)) => Ok((
                Self::Boolean(constant_1.not_equals(constant_2)),
                GeneratorExpressionOperator::not_equals(),
            )),
            (Self::Boolean(_), constant_2) => {
                Err(Error::OperatorNotEqualsSecondOperandExpectedBoolean {
                    location: constant_2.location(),
                    found: constant_2.to_string(),
                })
            }
            (Self::Integer(constant_1), Self::Integer(constant_2)) => constant_1
                .not_equals(constant_2)
                .map(|(boolean, operator)| (Self::Boolean(boolean), operator)),
            (Self::Integer(_), constant_2) => {
                Err(Error::OperatorNotEqualsSecondOperandExpectedInteger {
                    location: constant_2.location(),
                    found: constant_2.to_string(),
                })
            }
            (constant_1, _) => Err(Error::OperatorNotEqualsFirstOperandExpectedPrimitiveType {
                location: constant_1.location(),
                found: constant_1.to_string(),
            }),
        }
    }

    ///
    /// Executes the `>=` greater-equals comparison operator.
    ///
    pub fn greater_equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .greater_equals(integer_2)
                    .map(|(boolean, operator)| (Self::Boolean(boolean), operator)),
                constant => Err(Error::OperatorGreaterEqualsSecondOperandExpectedInteger {
                    location: constant.location(),
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorGreaterEqualsFirstOperandExpectedInteger {
                location: constant.location(),
                found: constant.to_string(),
            }),
        }
    }

    ///
    /// Executes the `<=` lesser-equals comparison operator.
    ///
    pub fn lesser_equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .lesser_equals(integer_2)
                    .map(|(boolean, operator)| (Self::Boolean(boolean), operator)),
                constant => Err(Error::OperatorLesserEqualsSecondOperandExpectedInteger {
                    location: constant.location(),
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorLesserEqualsFirstOperandExpectedInteger {
                location: constant.location(),
                found: constant.to_string(),
            }),
        }
    }

    ///
    /// Executes the `>` greater comparison operator.
    ///
    pub fn greater(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .greater(integer_2)
                    .map(|(boolean, operator)| (Self::Boolean(boolean), operator)),
                constant => Err(Error::OperatorGreaterSecondOperandExpectedInteger {
                    location: constant.location(),
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorGreaterFirstOperandExpectedInteger {
                location: constant.location(),
                found: constant.to_string(),
            }),
        }
    }

    ///
    /// Executes the `<` lesser comparison operator.
    ///
    pub fn lesser(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .lesser(integer_2)
                    .map(|(boolean, operator)| (Self::Boolean(boolean), operator)),
                constant => Err(Error::OperatorLesserSecondOperandExpectedInteger {
                    location: constant.location(),
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorLesserFirstOperandExpectedInteger {
                location: constant.location(),
                found: constant.to_string(),
            }),
        }
    }
}

impl BitOr for Constant {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn bitor(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .bitor(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator)),
                constant => Err(Error::OperatorBitwiseOrSecondOperandExpectedInteger {
                    location: constant.location(),
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorBitwiseOrFirstOperandExpectedInteger {
                location: constant.location(),
                found: constant.to_string(),
            }),
        }
    }
}

impl BitXor for Constant {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn bitxor(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .bitxor(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator)),
                constant => Err(Error::OperatorBitwiseXorSecondOperandExpectedInteger {
                    location: constant.location(),
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorBitwiseXorFirstOperandExpectedInteger {
                location: constant.location(),
                found: constant.to_string(),
            }),
        }
    }
}

impl BitAnd for Constant {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn bitand(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .bitand(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator)),
                constant => Err(Error::OperatorBitwiseAndSecondOperandExpectedInteger {
                    location: constant.location(),
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorBitwiseAndFirstOperandExpectedInteger {
                location: constant.location(),
                found: constant.to_string(),
            }),
        }
    }
}

impl Shl<Self> for Constant {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn shl(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .shl(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator)),
                constant => Err(
                    Error::OperatorBitwiseShiftLeftSecondOperandExpectedInteger {
                        location: constant.location(),
                        found: constant.to_string(),
                    },
                ),
            },
            constant => Err(Error::OperatorBitwiseShiftLeftFirstOperandExpectedInteger {
                location: constant.location(),
                found: constant.to_string(),
            }),
        }
    }
}

impl Shr<Self> for Constant {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn shr(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .shr(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator)),
                constant => Err(
                    Error::OperatorBitwiseShiftRightSecondOperandExpectedInteger {
                        location: constant.location(),
                        found: constant.to_string(),
                    },
                ),
            },
            constant => Err(
                Error::OperatorBitwiseShiftRightFirstOperandExpectedInteger {
                    location: constant.location(),
                    found: constant.to_string(),
                },
            ),
        }
    }
}

impl Add for Constant {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn add(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .add(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator)),
                constant => Err(Error::OperatorAdditionSecondOperandExpectedInteger {
                    location: constant.location(),
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorAdditionFirstOperandExpectedInteger {
                location: constant.location(),
                found: constant.to_string(),
            }),
        }
    }
}

impl Sub for Constant {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn sub(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .sub(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator)),
                constant => Err(Error::OperatorSubtractionSecondOperandExpectedInteger {
                    location: constant.location(),
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorSubtractionFirstOperandExpectedInteger {
                location: constant.location(),
                found: constant.to_string(),
            }),
        }
    }
}

impl Mul for Constant {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn mul(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .mul(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator)),
                constant => Err(Error::OperatorMultiplicationSecondOperandExpectedInteger {
                    location: constant.location(),
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorMultiplicationFirstOperandExpectedInteger {
                location: constant.location(),
                found: constant.to_string(),
            }),
        }
    }
}

impl Div for Constant {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn div(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .div(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator)),
                constant => Err(Error::OperatorDivisionSecondOperandExpectedInteger {
                    location: constant.location(),
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorDivisionFirstOperandExpectedInteger {
                location: constant.location(),
                found: constant.to_string(),
            }),
        }
    }
}

impl Rem for Constant {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn rem(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .rem(integer_2)
                    .map(|(integer, operator)| (Self::Integer(integer), operator)),
                constant => Err(Error::OperatorRemainderSecondOperandExpectedInteger {
                    location: constant.location(),
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorRemainderFirstOperandExpectedInteger {
                location: constant.location(),
                found: constant.to_string(),
            }),
        }
    }
}

impl Constant {
    ///
    /// Executes the `as` casting operator.
    ///
    pub fn cast(self, to: Type) -> Result<(Self, Option<GeneratorExpressionOperator>), Error> {
        let from = self.r#type();
        Caster::cast(&from, &to).map_err(|error| Error::OperatorCastingTypesMismatch {
            location: self.location(),
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
            Self::Boolean(constant) => Ok((
                Self::Boolean(constant.not()),
                GeneratorExpressionOperator::Not,
            )),
            constant => Err(Error::OperatorNotExpectedBoolean {
                location: constant.location(),
                found: constant.to_string(),
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
            constant => Err(Error::OperatorBitwiseNotExpectedInteger {
                location: constant.location(),
                found: constant.to_string(),
            }),
        }
    }
}

impl Neg for Constant {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn neg(self) -> Self::Output {
        match self {
            Self::Integer(integer) => integer
                .neg()
                .map(|(integer, operator)| (Self::Integer(integer), operator)),
            constant => Err(Error::OperatorNegationExpectedInteger {
                location: constant.location(),
                found: constant.to_string(),
            }),
        }
    }
}

impl Constant {
    ///
    /// Executes the `[]` array index operator with a non-constant value.
    ///
    pub fn index_value(self, other: Value) -> Result<(Element, IndexAccess), Error> {
        match self {
            Constant::Array(array) => match other {
                Value::Integer(_) => array.slice_single(None),
                value => Err(Error::OperatorIndexSecondOperandExpectedIntegerOrRange {
                    location: value
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: value.to_string(),
                }),
            },
            constant => Err(Error::OperatorIndexFirstOperandExpectedArray {
                location: constant.location(),
                found: constant.to_string(),
            }),
        }
    }

    ///
    /// Executes the `[]` array index operator with a constant value.
    ///
    pub fn index_constant(self, other: Constant) -> Result<(Element, IndexAccess), Error> {
        match self {
            Constant::Array(array) => match other {
                Constant::Integer(integer) => array.slice_single(Some(integer)),
                Constant::Range(range) => array
                    .slice_range(range)
                    .map(|(constant, access)| (Element::Constant(constant), access)),
                Constant::RangeInclusive(range) => array
                    .slice_range_inclusive(range)
                    .map(|(constant, access)| (Element::Constant(constant), access)),
                constant => Err(Error::OperatorIndexSecondOperandExpectedIntegerOrRange {
                    location: constant.location(),
                    found: constant.to_string(),
                }),
            },
            constant => Err(Error::OperatorIndexFirstOperandExpectedArray {
                location: constant.location(),
                found: constant.to_string(),
            }),
        }
    }

    ///
    /// Executes the `.` dot field access operator for a tuple.
    ///
    pub fn tuple_field(self, index: TupleIndex) -> Result<(Self, StackFieldAccess), Error> {
        match self {
            Constant::Tuple(tuple) => tuple.slice(index),
            constant => Err(Error::OperatorDotFirstOperandExpectedTuple {
                location: constant.location(),
                found: constant.to_string(),
            }),
        }
    }

    ///
    /// Executes the `.` dot field access operator for a structure.
    ///
    pub fn structure_field(
        self,
        identifier: Identifier,
    ) -> Result<(Self, StackFieldAccess), Error> {
        match self {
            Constant::Structure(structure) => structure.slice(identifier),
            constant => Err(Error::OperatorDotFirstOperandExpectedInstance {
                location: constant.location(),
                found: constant.to_string(),
            }),
        }
    }

    ///
    /// Sets the constant location in the code.
    ///
    pub fn set_location(&mut self, value: Location) {
        match self {
            Self::Unit(inner) => inner.location = value,
            Self::Boolean(inner) => inner.location = value,
            Self::Integer(inner) => inner.location = value,
            Self::Range(inner) => inner.location = value,
            Self::RangeInclusive(inner) => inner.location = value,
            Self::String(inner) => inner.location = value,
            Self::Array(inner) => inner.location = value,
            Self::Tuple(inner) => inner.location = value,
            Self::Structure(inner) => inner.location = value,
        }
    }

    ///
    /// Returns the constant location in the code.
    ///
    pub fn location(&self) -> Location {
        match self {
            Self::Unit(inner) => inner.location,
            Self::Boolean(inner) => inner.location,
            Self::Integer(inner) => inner.location,
            Self::Range(inner) => inner.location,
            Self::RangeInclusive(inner) => inner.location,
            Self::String(inner) => inner.location,
            Self::Array(inner) => inner.location,
            Self::Tuple(inner) => inner.location,
            Self::Structure(inner) => inner.location,
        }
    }

    ///
    /// If the constant was created from a literal.
    ///
    pub fn is_literal(&self) -> bool {
        match self {
            Self::Integer(inner) => inner.is_literal,
            _ => false,
        }
    }
}

impl ITyped for Constant {
    fn r#type(&self) -> Type {
        match self {
            Self::Unit(inner) => inner.r#type(),
            Self::Boolean(inner) => inner.r#type(),
            Self::Integer(inner) => inner.r#type(),
            Self::Range(inner) => inner.r#type(),
            Self::RangeInclusive(inner) => inner.r#type(),
            Self::String(inner) => inner.r#type(),
            Self::Array(inner) => inner.r#type(),
            Self::Tuple(inner) => inner.r#type(),
            Self::Structure(inner) => inner.r#type(),
        }
    }

    fn has_the_same_type_as(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Unit(inner_1), Self::Unit(inner_2)) => inner_1.has_the_same_type_as(inner_2),
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
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unit(inner) => write!(f, "unit {}", inner),
            Self::Boolean(inner) => write!(f, "boolean {}", inner),
            Self::Integer(inner) => write!(f, "integer {}", inner),
            Self::Range(inner) => write!(f, "range {}", inner),
            Self::RangeInclusive(inner) => write!(f, "range inclusive {}", inner),
            Self::String(inner) => write!(f, "string {}", inner),
            Self::Array(inner) => write!(f, "array {}", inner),
            Self::Tuple(inner) => write!(f, "tuple {}", inner),
            Self::Structure(inner) => write!(f, "structure {}", inner),
        }
    }
}
