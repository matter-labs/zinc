//!
//! The semantic analyzer constant element.
//!

mod error;
mod integer;

pub use self::error::Error;
pub use self::integer::Error as IntegerError;
pub use self::integer::Integer;

use std::fmt;

use num_bigint::BigInt;
use num_traits::One;
use num_traits::Zero;

use zinc_bytecode::Instruction;

use crate::semantic::Caster;
use crate::semantic::Type;
use crate::syntax::BooleanLiteral;

#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    Unit,
    Boolean(bool),
    Integer(Integer),
    String(String),
}

impl Constant {
    pub fn r#type(&self) -> Type {
        match self {
            Self::Unit => Type::new_unit(),
            Self::Boolean(_) => Type::new_boolean(),
            Self::Integer(integer) => integer.r#type(),
            Self::String(_) => Type::new_string(),
        }
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Unit, Self::Unit) => true,
            (Self::Boolean(_), Self::Boolean(_)) => true,
            (Self::Integer(value_1), Self::Integer(value_2)) => {
                value_1.has_the_same_type_as(value_2)
            }
            _ => false,
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
                    constant.to_owned(),
                )),
            },
            constant => Err(Error::OperatorOrFirstOperandExpectedBoolean(
                constant.to_owned(),
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
                    constant.to_owned(),
                )),
            },
            constant => Err(Error::OperatorXorFirstOperandExpectedBoolean(
                constant.to_owned(),
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
                    constant.to_owned(),
                )),
            },
            constant => Err(Error::OperatorAndFirstOperandExpectedBoolean(
                constant.to_owned(),
            )),
        }
    }

    pub fn equals(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Self::Unit, Self::Unit) => Ok(Self::Boolean(true)),
            (Self::Unit, value_2) => Err(Error::OperatorEqualsSecondOperandExpectedUnit(
                value_2.to_owned(),
            )),
            (Self::Boolean(value_1), Self::Boolean(value_2)) => {
                let result = value_1 == value_2;
                Ok(Self::Boolean(result))
            }
            (Self::Boolean(_), value_2) => Err(Error::OperatorEqualsSecondOperandExpectedBoolean(
                value_2.to_owned(),
            )),
            (Self::Integer(value_1), Self::Integer(value_2)) => {
                let result = value_1.equals(value_2).map_err(Error::Integer)?;
                Ok(Self::Boolean(result))
            }
            (Self::Integer(_), value_2) => Err(Error::OperatorEqualsSecondOperandExpectedInteger(
                value_2.to_owned(),
            )),
            (value_1, _) => Err(Error::OperatorEqualsFirstOperandExpectedPrimitiveType(
                value_1.to_owned(),
            )),
        }
    }

    pub fn not_equals(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Self::Unit, Self::Unit) => Ok(Self::Boolean(true)),
            (Self::Unit, value_2) => Err(Error::OperatorNotEqualsSecondOperandExpectedUnit(
                value_2.to_owned(),
            )),
            (Self::Boolean(value_1), Self::Boolean(value_2)) => {
                let result = value_1 != value_2;
                Ok(Self::Boolean(result))
            }
            (Self::Boolean(_), value_2) => Err(
                Error::OperatorNotEqualsSecondOperandExpectedBoolean(value_2.to_owned()),
            ),
            (Self::Integer(value_1), Self::Integer(value_2)) => {
                let result = value_1.not_equals(value_2).map_err(Error::Integer)?;
                Ok(Self::Boolean(result))
            }
            (Self::Integer(_), value_2) => Err(
                Error::OperatorNotEqualsSecondOperandExpectedInteger(value_2.to_owned()),
            ),
            (value_1, _) => Err(Error::OperatorNotEqualsFirstOperandExpectedPrimitiveType(
                value_1.to_owned(),
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
                    value.to_owned(),
                )),
            },
            value => Err(Error::OperatorGreaterEqualsFirstOperandExpectedInteger(
                value.to_owned(),
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
                    value.to_owned(),
                )),
            },
            value => Err(Error::OperatorLesserEqualsFirstOperandExpectedInteger(
                value.to_owned(),
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
                    value.to_owned(),
                )),
            },
            value => Err(Error::OperatorGreaterFirstOperandExpectedInteger(
                value.to_owned(),
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
                    value.to_owned(),
                )),
            },
            value => Err(Error::OperatorLesserFirstOperandExpectedInteger(
                value.to_owned(),
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
                    value.to_owned(),
                )),
            },
            value => Err(Error::OperatorAdditionFirstOperandExpectedInteger(
                value.to_owned(),
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
                    value.to_owned(),
                )),
            },
            value => Err(Error::OperatorSubtractionFirstOperandExpectedInteger(
                value.to_owned(),
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
                    value.to_owned(),
                )),
            },
            value => Err(Error::OperatorMultiplicationFirstOperandExpectedInteger(
                value.to_owned(),
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
                    value.to_owned(),
                )),
            },
            value => Err(Error::OperatorDivisionFirstOperandExpectedInteger(
                value.to_owned(),
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
                    value.to_owned(),
                )),
            },
            value => Err(Error::OperatorRemainderFirstOperandExpectedInteger(
                value.to_owned(),
            )),
        }
    }

    pub fn negate(&self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer) => integer.negate().map(Self::Integer).map_err(Error::Integer),
            value => Err(Error::OperatorNegationExpectedInteger(value.to_owned())),
        }
    }

    pub fn not(&self) -> Result<Self, Error> {
        match self {
            Self::Boolean(value) => {
                let result = !value;
                Ok(Self::Boolean(result))
            }
            value => Err(Error::OperatorNotExpectedBoolean(value.to_owned())),
        }
    }

    pub fn cast(&mut self, to: &Type) -> Result<Option<(bool, usize)>, Error> {
        let from = self.r#type();
        Caster::validate(&from, &to).map_err(Error::Casting)?;

        let (is_signed, bitlength) = match to {
            Type::IntegerUnsigned { bitlength } => (false, *bitlength),
            Type::IntegerSigned { bitlength } => (true, *bitlength),
            Type::Field => (false, crate::BITLENGTH_FIELD),
            _ => return Ok(None),
        };

        if let Self::Integer(integer) = self {
            integer.cast(is_signed, bitlength);
        }
        Ok(Some((is_signed, bitlength)))
    }
}

impl Into<Instruction> for Constant {
    fn into(self) -> Instruction {
        match self {
            Self::Unit => Instruction::NoOperation(zinc_bytecode::NoOperation),
            Self::Boolean(boolean) => Instruction::PushConst(zinc_bytecode::PushConst::new(
                if boolean {
                    BigInt::one()
                } else {
                    BigInt::zero()
                },
                false,
                crate::BITLENGTH_BOOLEAN,
            )),
            Self::Integer(integer) => integer.into(),
            Self::String(_) => Instruction::NoOperation(zinc_bytecode::NoOperation),
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

impl From<(usize, usize)> for Constant {
    fn from((value, bitlength): (usize, usize)) -> Self {
        Self::Integer(Integer::from((value, bitlength)))
    }
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unit => write!(f, "()"),
            Self::Boolean(boolean) => write!(f, "{}", boolean),
            Self::Integer(integer) => write!(f, "{}", integer),
            Self::String(string) => write!(f, "{}", string),
        }
    }
}
