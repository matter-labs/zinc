//!
//! The semantic analyzer element.
//!

mod tests;

pub mod access;
pub mod constant;
pub mod error;
pub mod path;
pub mod place;
pub mod r#type;
pub mod value;

use std::convert::TryFrom;
use std::fmt;

use crate::syntax::MemberString;

use self::access::AccessData;
use self::constant::Constant;
use self::error::Error;
use self::path::Path;
use self::place::Place;
use self::r#type::Type;
use self::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    /// Runtime value, which is unknown at compile time (`rvalue`)
    Value(Value),
    /// Constant value, which is known at compile time (`rvalue`)
    Constant(Constant),
    /// The second operand of the casting operator
    Type(Type),
    /// The second operand of the function call operator
    ArgumentList(Vec<Self>),

    /// Path to be resolved in the scope
    Path(Path),
    /// Memory descriptor (`lvalue`)
    Place(Place),
    /// Tuple field index
    MemberInteger(usize),
    /// Structure field name
    MemberString(MemberString),
    /// Module name
    Module(String),
}

impl Element {
    pub fn assign(self, other: &Self) -> Result<Place, Error> {
        match other {
            Self::Value(_) => {}
            Self::Constant(_) => {}
            element => {
                return Err(Error::OperatorAssignmentSecondOperandExpectedEvaluable(
                    element.to_string(),
                ))
            }
        }

        match self {
            Self::Place(place) => Ok(place),
            element => Err(Error::OperatorAssignmentFirstOperandExpectedPlace(
                element.to_string(),
            )),
        }
    }

    pub fn assign_add(self, other: &Self) -> Result<Place, Error> {
        match self {
            Self::Place(place) => {
                let value_1 = Value::try_from(&place.r#type).map_err(Error::Value)?;
                match other {
                    Self::Value(value_2) => {
                        value_1
                            .add(value_2)
                            .map(Self::Value)
                            .map_err(Error::Value)?;
                        Ok(place)
                    }
                    Self::Constant(value_2) => {
                        value_1
                            .add(&Value::try_from(value_2).map_err(Error::Value)?)
                            .map(Self::Value)
                            .map_err(Error::Value)?;
                        Ok(place)
                    }
                    element => Err(
                        Error::OperatorAssignmentAdditionSecondOperandExpectedEvaluable(
                            element.to_string(),
                        ),
                    ),
                }
            }
            element => Err(Error::OperatorAssignmentAdditionFirstOperandExpectedPlace(
                element.to_string(),
            )),
        }
    }

    pub fn assign_subtract(self, other: &Self) -> Result<Place, Error> {
        match self {
            Self::Place(place) => {
                let value_1 = Value::try_from(&place.r#type).map_err(Error::Value)?;
                match other {
                    Self::Value(value_2) => {
                        value_1
                            .subtract(value_2)
                            .map(Self::Value)
                            .map_err(Error::Value)?;
                        Ok(place)
                    }
                    Self::Constant(value_2) => {
                        value_1
                            .subtract(&Value::try_from(value_2).map_err(Error::Value)?)
                            .map(Self::Value)
                            .map_err(Error::Value)?;
                        Ok(place)
                    }
                    element => Err(
                        Error::OperatorAssignmentSubtractionSecondOperandExpectedEvaluable(
                            element.to_string(),
                        ),
                    ),
                }
            }
            element => Err(
                Error::OperatorAssignmentSubtractionFirstOperandExpectedPlace(element.to_string()),
            ),
        }
    }

    pub fn assign_multiply(self, other: &Self) -> Result<Place, Error> {
        match self {
            Self::Place(place) => {
                let value_1 = Value::try_from(&place.r#type).map_err(Error::Value)?;
                match other {
                    Self::Value(value_2) => {
                        value_1
                            .multiply(value_2)
                            .map(Self::Value)
                            .map_err(Error::Value)?;
                        Ok(place)
                    }
                    Self::Constant(value_2) => {
                        value_1
                            .multiply(&Value::try_from(value_2).map_err(Error::Value)?)
                            .map(Self::Value)
                            .map_err(Error::Value)?;
                        Ok(place)
                    }
                    element => Err(
                        Error::OperatorAssignmentMultiplicationSecondOperandExpectedEvaluable(
                            element.to_string(),
                        ),
                    ),
                }
            }
            element => Err(
                Error::OperatorAssignmentMultiplicationFirstOperandExpectedPlace(
                    element.to_string(),
                ),
            ),
        }
    }

    pub fn assign_divide(self, other: &Self) -> Result<Place, Error> {
        match self {
            Self::Place(place) => {
                let value_1 = Value::try_from(&place.r#type).map_err(Error::Value)?;
                match other {
                    Self::Value(value_2) => {
                        value_1
                            .divide(value_2)
                            .map(Self::Value)
                            .map_err(Error::Value)?;
                        Ok(place)
                    }
                    Self::Constant(value_2) => {
                        value_1
                            .divide(&Value::try_from(value_2).map_err(Error::Value)?)
                            .map(Self::Value)
                            .map_err(Error::Value)?;
                        Ok(place)
                    }
                    element => Err(
                        Error::OperatorAssignmentDivisionSecondOperandExpectedEvaluable(
                            element.to_string(),
                        ),
                    ),
                }
            }
            element => Err(Error::OperatorAssignmentDivisionFirstOperandExpectedPlace(
                element.to_string(),
            )),
        }
    }

    pub fn assign_remainder(self, other: &Self) -> Result<Place, Error> {
        match self {
            Self::Place(place) => {
                let value_1 = Value::try_from(&place.r#type).map_err(Error::Value)?;
                match other {
                    Self::Value(value_2) => {
                        value_1
                            .remainder(value_2)
                            .map(Self::Value)
                            .map_err(Error::Value)?;
                        Ok(place)
                    }
                    Self::Constant(value_2) => {
                        value_1
                            .remainder(&Value::try_from(value_2).map_err(Error::Value)?)
                            .map(Self::Value)
                            .map_err(Error::Value)?;
                        Ok(place)
                    }
                    element => Err(
                        Error::OperatorAssignmentRemainderSecondOperandExpectedEvaluable(
                            element.to_string(),
                        ),
                    ),
                }
            }
            element => Err(Error::OperatorAssignmentRemainderFirstOperandExpectedPlace(
                element.to_string(),
            )),
        }
    }

    pub fn range_inclusive(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .range_inclusive(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorRangeInclusiveSecondOperandExpectedConstant(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorRangeInclusiveFirstOperandExpectedConstant(
                element_1.to_string(),
            )),
        }
    }

    pub fn range(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .range(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorRangeSecondOperandExpectedConstant(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorRangeFirstOperandExpectedConstant(
                element_1.to_string(),
            )),
        }
    }

    pub fn or(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => {
                value_1.or(value_2).map(Self::Value).map_err(Error::Value)
            }
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .or(&Value::try_from(value_2).map_err(Error::Value)?)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(Error::OperatorOrSecondOperandExpectedEvaluable(
                element_2.to_string(),
            )),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .or(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .or(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorOrSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorOrFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn xor(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => {
                value_1.xor(value_2).map(Self::Value).map_err(Error::Value)
            }
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .xor(&Value::try_from(value_2).map_err(Error::Value)?)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorXorSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .xor(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .xor(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorXorSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorXorFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn and(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => {
                value_1.and(value_2).map(Self::Value).map_err(Error::Value)
            }
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .and(&Value::try_from(value_2).map_err(Error::Value)?)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorAndSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .and(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .and(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorAndSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorAndFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn equals(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .equals(value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .equals(&Value::try_from(value_2).map_err(Error::Value)?)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorEqualsSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .equals(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .equals(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorEqualsSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorEqualsFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn not_equals(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .not_equals(value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .not_equals(&Value::try_from(value_2).map_err(Error::Value)?)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorNotEqualsSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .not_equals(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .not_equals(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorNotEqualsSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorNotEqualsFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn greater_equals(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .greater_equals(value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .greater_equals(&Value::try_from(value_2).map_err(Error::Value)?)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorGreaterEqualsSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .greater_equals(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .greater_equals(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorGreaterEqualsSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorGreaterEqualsFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn lesser_equals(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .lesser_equals(value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .lesser_equals(&Value::try_from(value_2).map_err(Error::Value)?)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorLesserEqualsSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .lesser_equals(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .lesser_equals(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorLesserEqualsSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorLesserEqualsFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn greater(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .greater(value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .greater(&Value::try_from(value_2).map_err(Error::Value)?)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorGreaterSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .greater(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .greater(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorGreaterSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorGreaterFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn lesser(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .lesser(value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .lesser(&Value::try_from(value_2).map_err(Error::Value)?)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorLesserSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .lesser(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .lesser(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorLesserSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorLesserFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn add(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => {
                value_1.add(value_2).map(Self::Value).map_err(Error::Value)
            }
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .add(&Value::try_from(value_2).map_err(Error::Value)?)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorAdditionSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .add(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .add(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorAdditionSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorAdditionFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn subtract(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .subtract(value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .subtract(&Value::try_from(value_2).map_err(Error::Value)?)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorSubtractionSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .subtract(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .subtract(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorSubtractionSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorSubtractionFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn multiply(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .multiply(value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .multiply(&Value::try_from(value_2).map_err(Error::Value)?)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorMultiplicationSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .multiply(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .multiply(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorMultiplicationSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorMultiplicationFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn divide(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .divide(value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .divide(&Value::try_from(value_2).map_err(Error::Value)?)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorDivisionSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .divide(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .divide(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorDivisionSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorDivisionFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn remainder(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .remainder(value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .remainder(&Value::try_from(value_2).map_err(Error::Value)?)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorRemainderSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .remainder(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .remainder(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorRemainderSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorRemainderFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn cast(&mut self, other: &Self) -> Result<Option<(bool, usize)>, Error> {
        let r#type = match other {
            Self::Type(r#type) => r#type,
            element => {
                return Err(Error::OperatorCastingSecondOperandExpectedType(
                    element.to_string(),
                ))
            }
        };

        match self {
            Element::Value(value) => value.cast(r#type).map_err(Error::Value),
            Element::Constant(constant) => constant.cast(r#type).map_err(Error::Constant),
            element => Err(Error::OperatorCastingFirstOperandExpectedEvaluable(
                element.to_string(),
            )),
        }
    }

    pub fn negate(&self) -> Result<Self, Error> {
        match self {
            Element::Value(value) => value.negate().map(Self::Value).map_err(Error::Value),
            Element::Constant(constant) => constant
                .negate()
                .map(Self::Constant)
                .map_err(Error::Constant),
            element => Err(Error::OperatorNegationExpectedEvaluable(
                element.to_string(),
            )),
        }
    }

    pub fn not(&self) -> Result<Self, Error> {
        match self {
            Element::Value(value) => value.not().map(Self::Value).map_err(Error::Value),
            Element::Constant(constant) => {
                constant.not().map(Self::Constant).map_err(Error::Constant)
            }
            element => Err(Error::OperatorNotExpectedEvaluable(element.to_string())),
        }
    }

    pub fn index(&mut self, other: &Self) -> Result<AccessData, Error> {
        match self {
            Self::Place(place) => match other {
                element @ Self::Value(_) => place.index(element).map_err(Error::Place),
                element @ Self::Constant(_) => place.index(element).map_err(Error::Place),
                element => Err(Error::OperatorIndexSecondOperandExpectedEvaluable(
                    element.to_string(),
                )),
            },
            Self::Value(value) => match other {
                Self::Value(index) => value.index_value(index).map_err(Error::Value),
                Self::Constant(index) => value.index_constant(index).map_err(Error::Value),
                element => Err(Error::OperatorIndexSecondOperandExpectedEvaluable(
                    element.to_string(),
                )),
            },
            element => Err(Error::OperatorIndexFirstOperandExpectedPlaceOrEvaluable(
                element.to_string(),
            )),
        }
    }

    pub fn field(&mut self, other: &Self) -> Result<AccessData, Error> {
        match self {
            Self::Place(place) => match other {
                Self::MemberInteger(member) => place.field_tuple(*member).map_err(Error::Place),
                Self::MemberString(member) => {
                    place.field_structure(&member.name).map_err(Error::Place)
                }
                element => Err(Error::OperatorFieldSecondOperandExpectedMember(
                    element.to_string(),
                )),
            },
            Self::Value(value) => match other {
                Self::MemberInteger(member) => value.field_tuple(*member).map_err(Error::Value),
                Self::MemberString(member) => {
                    value.field_structure(&member.name).map_err(Error::Value)
                }
                element => Err(Error::OperatorFieldSecondOperandExpectedMember(
                    element.to_string(),
                )),
            },
            element => Err(Error::OperatorFieldFirstOperandExpectedPlaceOrEvaluable(
                element.to_string(),
            )),
        }
    }

    pub fn path(&mut self, other: &Self) -> Result<(), Error> {
        let path = match self {
            Self::Path(path) => path,
            element => {
                return Err(Error::OperatorPathFirstOperandExpectedPath(
                    element.to_string(),
                ))
            }
        };

        let member = match other {
            Self::MemberString(member) => member,
            element => {
                return Err(Error::OperatorPathSecondOperandExpectedMemberString(
                    element.to_string(),
                ))
            }
        };

        path.push_element(member.to_owned());
        Ok(())
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Value(value) => write!(f, "{}", value),
            Self::Constant(constant) => write!(f, "{}", constant),
            Self::Type(r#type) => write!(f, "{}", r#type),
            Self::ArgumentList(values) => write!(
                f,
                "{}",
                values
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Path(path) => write!(f, "{}", path),
            Self::Place(place) => write!(f, "{}", place),
            Self::MemberInteger(integer) => write!(f, "{}", integer),
            Self::MemberString(member) => write!(f, "{}", member.name),
            Self::Module(name) => write!(f, "{}", name),
        }
    }
}
