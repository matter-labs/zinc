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

use crate::generator::expression::operator::Operator as GeneratorExpressionOperator;
use crate::syntax::tree::identifier::Identifier;

use self::access::Field as FieldAccess;
use self::access::Index as IndexAccess;
use self::constant::Constant;
use self::error::Error;
use self::path::Path;
use self::place::Place;
use self::r#type::Type;
use self::value::Value;

///
/// An evaluated element of the semantic anylyzer evaluation stack.
///
/// The analyzer stores each evaluated syntax element in the stack in order to check
/// whether the semantic meaning of expressions are satisfied.
///
#[derive(Debug, Clone)]
pub enum Element {
    /// Runtime value, which is unknown at compile-time (`rvalue`)
    Value(Value),
    /// Constant value, which is known at compile-time (`rvalue`)
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
    TupleIndex(usize),
    /// Structure field name
    Identifier(Identifier),
    /// Module name
    Module(String),
}

impl Element {
    pub fn assign(self, other: Self) -> Result<(Place, GeneratorExpressionOperator), Error> {
        match other {
            Self::Value(_) => {}
            Self::Constant(_) => {}
            element => {
                return Err(Error::OperatorAssignmentSecondOperandExpectedEvaluable {
                    found: element.to_string(),
                })
            }
        }

        match self {
            Self::Place(place) => Ok((place, GeneratorExpressionOperator::None)),
            element => Err(Error::OperatorAssignmentFirstOperandExpectedPlace {
                found: element.to_string(),
            }),
        }
    }

    pub fn assign_bitwise_or(
        self,
        other: Self,
    ) -> Result<(Place, GeneratorExpressionOperator), Error> {
        match self {
            Self::Place(place) => {
                let value_1 = Value::try_from(&place.r#type).map_err(Error::Value)?;
                match other {
                    Self::Value(value_2) => value_1
                        .bitwise_or(value_2)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    Self::Constant(value_2) => value_1
                        .bitwise_or(Value::try_from(value_2).map_err(Error::Value)?)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    element => Err(
                        Error::OperatorAssignmentBitwiseOrSecondOperandExpectedEvaluable {
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(
                Error::OperatorAssignmentBitwiseOrFirstOperandExpectedPlace {
                    found: element.to_string(),
                },
            ),
        }
    }

    pub fn assign_bitwise_xor(
        self,
        other: Self,
    ) -> Result<(Place, GeneratorExpressionOperator), Error> {
        match self {
            Self::Place(place) => {
                let value_1 = Value::try_from(&place.r#type).map_err(Error::Value)?;
                match other {
                    Self::Value(value_2) => value_1
                        .bitwise_xor(value_2)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    Self::Constant(value_2) => value_1
                        .bitwise_xor(Value::try_from(value_2).map_err(Error::Value)?)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    element => Err(
                        Error::OperatorAssignmentBitwiseXorSecondOperandExpectedEvaluable {
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(
                Error::OperatorAssignmentBitwiseXorFirstOperandExpectedPlace {
                    found: element.to_string(),
                },
            ),
        }
    }

    pub fn assign_bitwise_and(
        self,
        other: Self,
    ) -> Result<(Place, GeneratorExpressionOperator), Error> {
        match self {
            Self::Place(place) => {
                let value_1 = Value::try_from(&place.r#type).map_err(Error::Value)?;
                match other {
                    Self::Value(value_2) => value_1
                        .bitwise_and(value_2)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    Self::Constant(value_2) => value_1
                        .bitwise_and(Value::try_from(value_2).map_err(Error::Value)?)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    element => Err(
                        Error::OperatorAssignmentBitwiseAndSecondOperandExpectedEvaluable {
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(
                Error::OperatorAssignmentBitwiseAndFirstOperandExpectedPlace {
                    found: element.to_string(),
                },
            ),
        }
    }

    pub fn assign_bitwise_shift_left(
        self,
        other: Self,
    ) -> Result<(Place, GeneratorExpressionOperator), Error> {
        match self {
            Self::Place(place) => {
                let value_1 = Value::try_from(&place.r#type).map_err(Error::Value)?;
                match other {
                    Self::Value(value_2) => value_1
                        .bitwise_shift_left(value_2)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    Self::Constant(value_2) => value_1
                        .bitwise_shift_left(Value::try_from(value_2).map_err(Error::Value)?)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    element => Err(
                        Error::OperatorAssignmentBitwiseShiftLeftSecondOperandExpectedEvaluable {
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(
                Error::OperatorAssignmentBitwiseShiftLeftFirstOperandExpectedPlace {
                    found: element.to_string(),
                },
            ),
        }
    }

    pub fn assign_bitwise_shift_right(
        self,
        other: Self,
    ) -> Result<(Place, GeneratorExpressionOperator), Error> {
        match self {
            Self::Place(place) => {
                let value_1 = Value::try_from(&place.r#type).map_err(Error::Value)?;
                match other {
                    Self::Value(value_2) => value_1
                        .bitwise_shift_right(value_2)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    Self::Constant(value_2) => value_1
                        .bitwise_shift_right(Value::try_from(value_2).map_err(Error::Value)?)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    element => Err(
                        Error::OperatorAssignmentBitwiseShiftRightSecondOperandExpectedEvaluable {
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(
                Error::OperatorAssignmentBitwiseShiftRightFirstOperandExpectedPlace {
                    found: element.to_string(),
                },
            ),
        }
    }

    pub fn assign_add(self, other: Self) -> Result<(Place, GeneratorExpressionOperator), Error> {
        match self {
            Self::Place(place) => {
                let value_1 = Value::try_from(&place.r#type).map_err(Error::Value)?;
                match other {
                    Self::Value(value_2) => value_1
                        .add(value_2)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    Self::Constant(value_2) => value_1
                        .add(Value::try_from(value_2).map_err(Error::Value)?)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    element => Err(
                        Error::OperatorAssignmentAdditionSecondOperandExpectedEvaluable {
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(Error::OperatorAssignmentAdditionFirstOperandExpectedPlace {
                found: element.to_string(),
            }),
        }
    }

    pub fn assign_subtract(
        self,
        other: Self,
    ) -> Result<(Place, GeneratorExpressionOperator), Error> {
        match self {
            Self::Place(place) => {
                let value_1 = Value::try_from(&place.r#type).map_err(Error::Value)?;
                match other {
                    Self::Value(value_2) => value_1
                        .subtract(value_2)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    Self::Constant(value_2) => value_1
                        .subtract(Value::try_from(value_2).map_err(Error::Value)?)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    element => Err(
                        Error::OperatorAssignmentSubtractionSecondOperandExpectedEvaluable {
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(
                Error::OperatorAssignmentSubtractionFirstOperandExpectedPlace {
                    found: element.to_string(),
                },
            ),
        }
    }

    pub fn assign_multiply(
        self,
        other: Self,
    ) -> Result<(Place, GeneratorExpressionOperator), Error> {
        match self {
            Self::Place(place) => {
                let value_1 = Value::try_from(&place.r#type).map_err(Error::Value)?;
                match other {
                    Self::Value(value_2) => value_1
                        .multiply(value_2)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    Self::Constant(value_2) => value_1
                        .multiply(Value::try_from(value_2).map_err(Error::Value)?)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    element => Err(
                        Error::OperatorAssignmentMultiplicationSecondOperandExpectedEvaluable {
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(
                Error::OperatorAssignmentMultiplicationFirstOperandExpectedPlace {
                    found: element.to_string(),
                },
            ),
        }
    }

    pub fn assign_divide(self, other: Self) -> Result<(Place, GeneratorExpressionOperator), Error> {
        match self {
            Self::Place(place) => {
                let value_1 = Value::try_from(&place.r#type).map_err(Error::Value)?;
                match other {
                    Self::Value(value_2) => value_1
                        .divide(value_2)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    Self::Constant(value_2) => value_1
                        .divide(Value::try_from(value_2).map_err(Error::Value)?)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    element => Err(
                        Error::OperatorAssignmentDivisionSecondOperandExpectedEvaluable {
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(Error::OperatorAssignmentDivisionFirstOperandExpectedPlace {
                found: element.to_string(),
            }),
        }
    }

    pub fn assign_remainder(
        self,
        other: Self,
    ) -> Result<(Place, GeneratorExpressionOperator), Error> {
        match self {
            Self::Place(place) => {
                let value_1 = Value::try_from(&place.r#type).map_err(Error::Value)?;
                match other {
                    Self::Value(value_2) => value_1
                        .remainder(value_2)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    Self::Constant(value_2) => value_1
                        .remainder(Value::try_from(value_2).map_err(Error::Value)?)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    element => Err(
                        Error::OperatorAssignmentRemainderSecondOperandExpectedEvaluable {
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(
                Error::OperatorAssignmentRemainderFirstOperandExpectedPlace {
                    found: element.to_string(),
                },
            ),
        }
    }

    pub fn range_inclusive(self, other: Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .range_inclusive(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorRangeInclusiveSecondOperandExpectedConstant {
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorRangeInclusiveFirstOperandExpectedConstant {
                found: element_1.to_string(),
            }),
        }
    }

    pub fn range(self, other: Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .range(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorRangeSecondOperandExpectedConstant {
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorRangeFirstOperandExpectedConstant {
                found: element_1.to_string(),
            }),
        }
    }

    pub fn or(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .or(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .or(Value::try_from(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorOrSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .or(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .or(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorOrSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorOrFirstOperandExpectedEvaluable {
                found: element_1.to_string(),
            }),
        }
    }

    pub fn xor(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .xor(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .xor(Value::try_from(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorXorSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .xor(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .xor(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorXorSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorXorFirstOperandExpectedEvaluable {
                found: element_1.to_string(),
            }),
        }
    }

    pub fn and(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .and(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .and(Value::try_from(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorAndSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .and(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .and(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorAndSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorAndFirstOperandExpectedEvaluable {
                found: element_1.to_string(),
            }),
        }
    }

    pub fn equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .equals(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .equals(Value::try_from(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorEqualsSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .equals(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .equals(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorEqualsSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorEqualsFirstOperandExpectedEvaluable {
                found: element_1.to_string(),
            }),
        }
    }

    pub fn not_equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .not_equals(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .not_equals(Value::try_from(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorNotEqualsSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .not_equals(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .not_equals(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorNotEqualsSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorNotEqualsFirstOperandExpectedEvaluable {
                found: element_1.to_string(),
            }),
        }
    }

    pub fn greater_equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .greater_equals(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .greater_equals(Value::try_from(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorGreaterEqualsSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .greater_equals(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .greater_equals(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorGreaterEqualsSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorGreaterEqualsFirstOperandExpectedEvaluable {
                found: element_1.to_string(),
            }),
        }
    }

    pub fn lesser_equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .lesser_equals(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .lesser_equals(Value::try_from(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorLesserEqualsSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .lesser_equals(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .lesser_equals(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorLesserEqualsSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorLesserEqualsFirstOperandExpectedEvaluable {
                found: element_1.to_string(),
            }),
        }
    }

    pub fn greater(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .greater(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .greater(Value::try_from(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorGreaterSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .greater(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .greater(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorGreaterSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorGreaterFirstOperandExpectedEvaluable {
                found: element_1.to_string(),
            }),
        }
    }

    pub fn lesser(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .lesser(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .lesser(Value::try_from(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorLesserSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .lesser(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .lesser(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorLesserSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorLesserFirstOperandExpectedEvaluable {
                found: element_1.to_string(),
            }),
        }
    }

    pub fn bitwise_or(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .bitwise_or(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .bitwise_or(Value::try_from(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorBitwiseOrSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .bitwise_or(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .bitwise_or(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorBitwiseOrSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorBitwiseOrFirstOperandExpectedEvaluable {
                found: element_1.to_string(),
            }),
        }
    }

    pub fn bitwise_xor(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .bitwise_xor(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .bitwise_xor(Value::try_from(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorBitwiseXorSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .bitwise_xor(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .bitwise_xor(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorBitwiseXorSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorBitwiseXorFirstOperandExpectedEvaluable {
                found: element_1.to_string(),
            }),
        }
    }

    pub fn bitwise_and(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .bitwise_and(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .bitwise_and(Value::try_from(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorBitwiseAndSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .bitwise_and(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .bitwise_and(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorBitwiseAndSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorBitwiseAndFirstOperandExpectedEvaluable {
                found: element_1.to_string(),
            }),
        }
    }

    pub fn bitwise_shift_left(
        self,
        other: Self,
    ) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .bitwise_shift_left(Value::try_from(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorBitwiseShiftLeftSecondOperandExpectedConstant {
                    found: element_2.to_string(),
                },
            ),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .bitwise_shift_left(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorBitwiseShiftLeftSecondOperandExpectedConstant {
                    found: element_2.to_string(),
                },
            ),
            (element_1, _) => Err(
                Error::OperatorBitwiseShiftLeftFirstOperandExpectedEvaluable {
                    found: element_1.to_string(),
                },
            ),
        }
    }

    pub fn bitwise_shift_right(
        self,
        other: Self,
    ) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .bitwise_shift_right(Value::try_from(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorBitwiseShiftRightSecondOperandExpectedConstant {
                    found: element_2.to_string(),
                },
            ),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .bitwise_shift_right(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorBitwiseShiftRightSecondOperandExpectedConstant {
                    found: element_2.to_string(),
                },
            ),
            (element_1, _) => Err(
                Error::OperatorBitwiseShiftRightFirstOperandExpectedEvaluable {
                    found: element_1.to_string(),
                },
            ),
        }
    }

    pub fn add(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .add(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .add(Value::try_from(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorAdditionSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .add(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .add(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorAdditionSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorAdditionFirstOperandExpectedEvaluable {
                found: element_1.to_string(),
            }),
        }
    }

    pub fn subtract(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .subtract(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .subtract(Value::try_from(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorSubtractionSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .subtract(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .subtract(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorSubtractionSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorSubtractionFirstOperandExpectedEvaluable {
                found: element_1.to_string(),
            }),
        }
    }

    pub fn multiply(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .multiply(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .multiply(Value::try_from(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorMultiplicationSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                },
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .multiply(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .multiply(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorMultiplicationSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                },
            ),
            (element_1, _) => Err(Error::OperatorMultiplicationFirstOperandExpectedEvaluable {
                found: element_1.to_string(),
            }),
        }
    }

    pub fn divide(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .divide(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .divide(Value::try_from(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorDivisionSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .divide(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .divide(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorDivisionSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorDivisionFirstOperandExpectedEvaluable {
                found: element_1.to_string(),
            }),
        }
    }

    pub fn remainder(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .remainder(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .remainder(Value::try_from(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorRemainderSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => Value::try_from(value_1)
                .map_err(Error::Value)?
                .remainder(value_2)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .remainder(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorRemainderSecondOperandExpectedEvaluable {
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorRemainderFirstOperandExpectedEvaluable {
                found: element_1.to_string(),
            }),
        }
    }

    pub fn cast(self, other: Self) -> Result<(Self, Option<GeneratorExpressionOperator>), Error> {
        let r#type = match other {
            Self::Type(r#type) => r#type,
            element => {
                return Err(Error::OperatorCastingSecondOperandExpectedType {
                    found: element.to_string(),
                })
            }
        };

        match self {
            Element::Value(value) => value
                .cast(r#type)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            Element::Constant(constant) => constant
                .cast(r#type)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            element => Err(Error::OperatorCastingFirstOperandExpectedEvaluable {
                found: element.to_string(),
            }),
        }
    }

    pub fn not(self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Element::Value(value) => value
                .not()
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            Element::Constant(constant) => constant
                .not()
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            element => Err(Error::OperatorNotExpectedEvaluable {
                found: element.to_string(),
            }),
        }
    }

    pub fn bitwise_not(self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Element::Value(value) => value
                .bitwise_not()
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            Element::Constant(constant) => constant
                .bitwise_not()
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            element => Err(Error::OperatorBitwiseNotExpectedEvaluable {
                found: element.to_string(),
            }),
        }
    }

    pub fn negate(self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Element::Value(value) => value
                .negate()
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            Element::Constant(constant) => constant
                .negate()
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            element => Err(Error::OperatorNegationExpectedEvaluable {
                found: element.to_string(),
            }),
        }
    }

    pub fn index(self, other: Self) -> Result<(Self, IndexAccess), Error> {
        match self {
            Self::Place(place) => match other {
                element @ Self::Value(_) => place
                    .index(element)
                    .map(|(place, access)| (Element::Place(place), access))
                    .map_err(Error::Place),
                element @ Self::Constant(_) => place
                    .index(element)
                    .map(|(place, access)| (Element::Place(place), access))
                    .map_err(Error::Place),
                element => Err(Error::OperatorIndexSecondOperandExpectedEvaluable {
                    found: element.to_string(),
                }),
            },
            Self::Value(value) => match other {
                Self::Value(index) => value
                    .index_value(index)
                    .map(|(value, access)| (Element::Value(value), access))
                    .map_err(Error::Value),
                Self::Constant(index) => value
                    .index_constant(index)
                    .map(|(value, access)| (Element::Value(value), access))
                    .map_err(Error::Value),
                element => Err(Error::OperatorIndexSecondOperandExpectedEvaluable {
                    found: element.to_string(),
                }),
            },
            element => Err(Error::OperatorIndexFirstOperandExpectedPlaceOrEvaluable {
                found: element.to_string(),
            }),
        }
    }

    pub fn field(self, other: Self) -> Result<(Self, FieldAccess), Error> {
        match self {
            Self::Place(place) => match other {
                Self::TupleIndex(index) => place
                    .field_tuple(index)
                    .map(|(place, access)| (Element::Place(place), access))
                    .map_err(Error::Place),
                Self::Identifier(identifier) => place
                    .field_structure(identifier.name)
                    .map(|(place, access)| (Element::Place(place), access))
                    .map_err(Error::Place),
                element => Err(Error::OperatorFieldSecondOperandExpectedIdentifier {
                    found: element.to_string(),
                }),
            },
            Self::Value(value) => match other {
                Self::TupleIndex(index) => value
                    .field_tuple(index)
                    .map(|(value, access)| (Element::Value(value), access))
                    .map_err(Error::Value),
                Self::Identifier(identifier) => value
                    .field_structure(identifier.name)
                    .map(|(value, access)| (Element::Value(value), access))
                    .map_err(Error::Value),
                element => Err(Error::OperatorFieldSecondOperandExpectedIdentifier {
                    found: element.to_string(),
                }),
            },
            element => Err(Error::OperatorFieldFirstOperandExpectedPlaceOrEvaluable {
                found: element.to_string(),
            }),
        }
    }

    pub fn path(self, other: Self) -> Result<Self, Error> {
        let mut path = match self {
            Self::Path(path) => path,
            element => {
                return Err(Error::OperatorPathFirstOperandExpectedPath {
                    found: element.to_string(),
                })
            }
        };

        let identifier = match other {
            Self::Identifier(identifier) => identifier,
            element => {
                return Err(Error::OperatorPathSecondOperandExpectedIdentifier {
                    found: element.to_string(),
                })
            }
        };

        path.push_element(identifier);

        Ok(Self::Path(path))
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Value(inner) => write!(f, "{}", inner),
            Self::Constant(inner) => write!(f, "{}", inner),
            Self::Type(inner) => write!(f, "{}", inner),
            Self::ArgumentList(inner) => write!(
                f,
                "{}",
                inner
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Path(inner) => write!(f, "{}", inner),
            Self::Place(inner) => write!(f, "{}", inner),
            Self::TupleIndex(inner) => write!(f, "{}", inner),
            Self::Identifier(inner) => write!(f, "{}", inner.name),
            Self::Module(inner) => write!(f, "{}", inner),
        }
    }
}
