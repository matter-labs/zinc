//!
//! The semantic analyzer element.
//!

mod tests;

pub mod access;
pub mod argument_list;
pub mod constant;
pub mod error;
pub mod path;
pub mod place;
pub mod tuple_index;
pub mod r#type;
pub mod value;

use std::fmt;

use crate::generator::expression::operator::Operator as GeneratorExpressionOperator;
use crate::lexical::token::location::Location;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::error::Error as SemanticError;
use crate::semantic::scope::item::Item as ScopeItem;
use crate::syntax::tree::identifier::Identifier;

use self::access::dot::Dot as DotAccessVariant;
use self::access::index::Index as IndexAccess;
use self::argument_list::ArgumentList;
use self::constant::Constant;
use self::error::Error;
use self::path::Path;
use self::place::Place;
use self::r#type::Type;
use self::tuple_index::TupleIndex;
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
    ArgumentList(ArgumentList),

    /// Path to be defined in the scope
    Path(Path),
    /// Memory descriptor (`lvalue`)
    Place(Place),
    /// Tuple field index
    TupleIndex(TupleIndex),
    /// Structure field identifier
    Identifier(Identifier),
    /// Module identifier
    Module(Identifier),
}

impl Element {
    pub fn assign(self, other: Self) -> Result<(Place, GeneratorExpressionOperator), Error> {
        match other {
            Self::Value(_) => {}
            Self::Constant(_) => {}
            element => {
                return Err(Error::OperatorAssignmentSecondOperandExpectedEvaluable {
                    location: element
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element.to_string(),
                })
            }
        }

        match self {
            Self::Place(place) => Ok((place, GeneratorExpressionOperator::None)),
            element => Err(Error::OperatorAssignmentFirstOperandExpectedPlace {
                location: element
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                let value_1 = Value::try_from_place(&place).map_err(Error::Value)?;
                match other {
                    Self::Value(value_2) => value_1
                        .bitwise_or(value_2)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    Self::Constant(value_2) => value_1
                        .bitwise_or(Value::try_from_constant(value_2).map_err(Error::Value)?)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    element => Err(
                        Error::OperatorAssignmentBitwiseOrSecondOperandExpectedEvaluable {
                            location: element
                                .location()
                                .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(
                Error::OperatorAssignmentBitwiseOrFirstOperandExpectedPlace {
                    location: element
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                let value_1 = Value::try_from_place(&place).map_err(Error::Value)?;
                match other {
                    Self::Value(value_2) => value_1
                        .bitwise_xor(value_2)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    Self::Constant(value_2) => value_1
                        .bitwise_xor(Value::try_from_constant(value_2).map_err(Error::Value)?)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    element => Err(
                        Error::OperatorAssignmentBitwiseXorSecondOperandExpectedEvaluable {
                            location: element
                                .location()
                                .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(
                Error::OperatorAssignmentBitwiseXorFirstOperandExpectedPlace {
                    location: element
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                let value_1 = Value::try_from_place(&place).map_err(Error::Value)?;
                match other {
                    Self::Value(value_2) => value_1
                        .bitwise_and(value_2)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    Self::Constant(value_2) => value_1
                        .bitwise_and(Value::try_from_constant(value_2).map_err(Error::Value)?)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    element => Err(
                        Error::OperatorAssignmentBitwiseAndSecondOperandExpectedEvaluable {
                            location: element
                                .location()
                                .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(
                Error::OperatorAssignmentBitwiseAndFirstOperandExpectedPlace {
                    location: element
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                let value_1 = Value::try_from_place(&place).map_err(Error::Value)?;
                match other {
                    Self::Value(value_2) => value_1
                        .bitwise_shift_left(value_2)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    Self::Constant(value_2) => value_1
                        .bitwise_shift_left(
                            Value::try_from_constant(value_2).map_err(Error::Value)?,
                        )
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    element => Err(
                        Error::OperatorAssignmentBitwiseShiftLeftSecondOperandExpectedEvaluable {
                            location: element
                                .location()
                                .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(
                Error::OperatorAssignmentBitwiseShiftLeftFirstOperandExpectedPlace {
                    location: element
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                let value_1 = Value::try_from_place(&place).map_err(Error::Value)?;
                match other {
                    Self::Value(value_2) => value_1
                        .bitwise_shift_right(value_2)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    Self::Constant(value_2) => value_1
                        .bitwise_shift_right(
                            Value::try_from_constant(value_2).map_err(Error::Value)?,
                        )
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    element => Err(
                        Error::OperatorAssignmentBitwiseShiftRightSecondOperandExpectedEvaluable {
                            location: element
                                .location()
                                .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(
                Error::OperatorAssignmentBitwiseShiftRightFirstOperandExpectedPlace {
                    location: element
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element.to_string(),
                },
            ),
        }
    }

    pub fn assign_add(self, other: Self) -> Result<(Place, GeneratorExpressionOperator), Error> {
        match self {
            Self::Place(place) => {
                let value_1 = Value::try_from_place(&place).map_err(Error::Value)?;
                match other {
                    Self::Value(value_2) => value_1
                        .add(value_2)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    Self::Constant(value_2) => value_1
                        .add(Value::try_from_constant(value_2).map_err(Error::Value)?)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    element => Err(
                        Error::OperatorAssignmentAdditionSecondOperandExpectedEvaluable {
                            location: element
                                .location()
                                .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(Error::OperatorAssignmentAdditionFirstOperandExpectedPlace {
                location: element
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                let value_1 = Value::try_from_place(&place).map_err(Error::Value)?;
                match other {
                    Self::Value(value_2) => value_1
                        .subtract(value_2)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    Self::Constant(value_2) => value_1
                        .subtract(Value::try_from_constant(value_2).map_err(Error::Value)?)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    element => Err(
                        Error::OperatorAssignmentSubtractionSecondOperandExpectedEvaluable {
                            location: element
                                .location()
                                .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(
                Error::OperatorAssignmentSubtractionFirstOperandExpectedPlace {
                    location: element
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                let value_1 = Value::try_from_place(&place).map_err(Error::Value)?;
                match other {
                    Self::Value(value_2) => value_1
                        .multiply(value_2)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    Self::Constant(value_2) => value_1
                        .multiply(Value::try_from_constant(value_2).map_err(Error::Value)?)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    element => Err(
                        Error::OperatorAssignmentMultiplicationSecondOperandExpectedEvaluable {
                            location: element
                                .location()
                                .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(
                Error::OperatorAssignmentMultiplicationFirstOperandExpectedPlace {
                    location: element
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element.to_string(),
                },
            ),
        }
    }

    pub fn assign_divide(self, other: Self) -> Result<(Place, GeneratorExpressionOperator), Error> {
        match self {
            Self::Place(place) => {
                let value_1 = Value::try_from_place(&place).map_err(Error::Value)?;
                match other {
                    Self::Value(value_2) => value_1
                        .divide(value_2)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    Self::Constant(value_2) => value_1
                        .divide(Value::try_from_constant(value_2).map_err(Error::Value)?)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    element => Err(
                        Error::OperatorAssignmentDivisionSecondOperandExpectedEvaluable {
                            location: element
                                .location()
                                .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(Error::OperatorAssignmentDivisionFirstOperandExpectedPlace {
                location: element
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                let value_1 = Value::try_from_place(&place).map_err(Error::Value)?;
                match other {
                    Self::Value(value_2) => value_1
                        .remainder(value_2)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    Self::Constant(value_2) => value_1
                        .remainder(Value::try_from_constant(value_2).map_err(Error::Value)?)
                        .map(|(_value, operator)| (place, operator))
                        .map_err(Error::Value),
                    element => Err(
                        Error::OperatorAssignmentRemainderSecondOperandExpectedEvaluable {
                            location: element
                                .location()
                                .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(
                Error::OperatorAssignmentRemainderFirstOperandExpectedPlace {
                    location: element
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorRangeInclusiveFirstOperandExpectedConstant {
                location: element_1
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorRangeFirstOperandExpectedConstant {
                location: element_1
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                .or(Value::try_from_constant(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorOrSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)
                    .map_err(Error::Value)?
                    .or(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
                    .map_err(Error::Value)
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .or(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorOrSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorOrFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                .xor(Value::try_from_constant(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorXorSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)
                    .map_err(Error::Value)?
                    .xor(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
                    .map_err(Error::Value)
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .xor(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorXorSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorXorFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                .and(Value::try_from_constant(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorAndSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)
                    .map_err(Error::Value)?
                    .and(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
                    .map_err(Error::Value)
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .and(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorAndSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorAndFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                .equals(Value::try_from_constant(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorEqualsSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)
                    .map_err(Error::Value)?
                    .equals(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
                    .map_err(Error::Value)
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .equals(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorEqualsSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorEqualsFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                .not_equals(Value::try_from_constant(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorNotEqualsSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)
                    .map_err(Error::Value)?
                    .not_equals(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
                    .map_err(Error::Value)
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .not_equals(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorNotEqualsSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorNotEqualsFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                .greater_equals(Value::try_from_constant(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorGreaterEqualsSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)
                    .map_err(Error::Value)?
                    .greater_equals(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
                    .map_err(Error::Value)
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .greater_equals(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorGreaterEqualsSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorGreaterEqualsFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                .lesser_equals(Value::try_from_constant(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorLesserEqualsSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)
                    .map_err(Error::Value)?
                    .lesser_equals(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
                    .map_err(Error::Value)
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .lesser_equals(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorLesserEqualsSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorLesserEqualsFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                .greater(Value::try_from_constant(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorGreaterSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)
                    .map_err(Error::Value)?
                    .greater(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
                    .map_err(Error::Value)
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .greater(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorGreaterSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorGreaterFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                .lesser(Value::try_from_constant(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorLesserSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)
                    .map_err(Error::Value)?
                    .lesser(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
                    .map_err(Error::Value)
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .lesser(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorLesserSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorLesserFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                .bitwise_or(Value::try_from_constant(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorBitwiseOrSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)
                    .map_err(Error::Value)?
                    .bitwise_or(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
                    .map_err(Error::Value)
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .bitwise_or(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorBitwiseOrSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorBitwiseOrFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                .bitwise_xor(Value::try_from_constant(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorBitwiseXorSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)
                    .map_err(Error::Value)?
                    .bitwise_xor(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
                    .map_err(Error::Value)
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .bitwise_xor(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorBitwiseXorSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorBitwiseXorFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                .bitwise_and(Value::try_from_constant(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorBitwiseAndSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)
                    .map_err(Error::Value)?
                    .bitwise_and(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
                    .map_err(Error::Value)
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .bitwise_and(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorBitwiseAndSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorBitwiseAndFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                .bitwise_shift_left(Value::try_from_constant(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorBitwiseShiftLeftSecondOperandExpectedConstant {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                },
            ),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .bitwise_shift_left(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorBitwiseShiftLeftSecondOperandExpectedConstant {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                },
            ),
            (element_1, _) => Err(
                Error::OperatorBitwiseShiftLeftFirstOperandExpectedEvaluable {
                    location: element_1
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                .bitwise_shift_right(Value::try_from_constant(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorBitwiseShiftRightSecondOperandExpectedConstant {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                },
            ),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .bitwise_shift_right(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorBitwiseShiftRightSecondOperandExpectedConstant {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                },
            ),
            (element_1, _) => Err(
                Error::OperatorBitwiseShiftRightFirstOperandExpectedEvaluable {
                    location: element_1
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                .add(Value::try_from_constant(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorAdditionSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)
                    .map_err(Error::Value)?
                    .add(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
                    .map_err(Error::Value)
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .add(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorAdditionSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorAdditionFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                .subtract(Value::try_from_constant(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorSubtractionSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)
                    .map_err(Error::Value)?
                    .subtract(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
                    .map_err(Error::Value)
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .subtract(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorSubtractionSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorSubtractionFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                .multiply(Value::try_from_constant(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorMultiplicationSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                },
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)
                    .map_err(Error::Value)?
                    .multiply(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
                    .map_err(Error::Value)
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .multiply(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorMultiplicationSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                },
            ),
            (element_1, _) => Err(Error::OperatorMultiplicationFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                .divide(Value::try_from_constant(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorDivisionSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)
                    .map_err(Error::Value)?
                    .divide(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
                    .map_err(Error::Value)
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .divide(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorDivisionSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorDivisionFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                .remainder(Value::try_from_constant(value_2).map_err(Error::Value)?)
                .map(|(value, operator)| (Self::Value(value), operator))
                .map_err(Error::Value),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorRemainderSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)
                    .map_err(Error::Value)?
                    .remainder(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
                    .map_err(Error::Value)
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .remainder(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator))
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorRemainderSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorRemainderFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                found: element_1.to_string(),
            }),
        }
    }

    pub fn cast(self, other: Self) -> Result<(Self, Option<GeneratorExpressionOperator>), Error> {
        let r#type = match other {
            Self::Type(r#type) => r#type,
            element => {
                return Err(Error::OperatorCastingSecondOperandExpectedType {
                    location: element
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                location: element
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                location: element
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                location: element
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                location: element
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                    location: element
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
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
                    location: element
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element.to_string(),
                }),
            },
            Self::Constant(constant) => match other {
                Self::Constant(index) => constant
                    .index(index)
                    .map(|(constant, access)| (Element::Constant(constant), access))
                    .map_err(Error::Constant),
                element => Err(Error::OperatorIndexSecondOperandExpectedEvaluable {
                    location: element
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element.to_string(),
                }),
            },
            element => Err(Error::OperatorIndexFirstOperandExpectedPlaceOrEvaluable {
                location: element
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                found: element.to_string(),
            }),
        }
    }

    pub fn dot(self, other: Self) -> Result<(Self, DotAccessVariant), SemanticError> {
        log::trace!("Executing the dot operation");

        match self {
            Self::Place(place) => match other {
                Self::TupleIndex(index) => place
                    .tuple_field(index)
                    .map(|(place, access)| (Element::Place(place), access))
                    .map_err(Error::Place)
                    .map_err(SemanticError::Element),
                Self::Identifier(identifier) => {
                    let scope = match place.r#type {
                        Type::Structure(ref inner) => inner.scope.to_owned(),
                        Type::Enumeration(ref inner) => inner.scope.to_owned(),
                        Type::Contract(ref inner) => inner.scope.to_owned(),
                        _ => {
                            return place
                                .structure_field(identifier)
                                .map(|(place, access)| (Element::Place(place), access))
                                .map_err(Error::Place)
                                .map_err(SemanticError::Element)
                        }
                    };

                    let item = scope.borrow().resolve_item(&identifier, false);
                    match item {
                        Ok(item) => match *item.borrow() {
                            ScopeItem::Type(ref r#type) => {
                                let r#type = r#type.define()?;
                                Ok((
                                    Element::Type(r#type),
                                    DotAccessVariant::Method {
                                        instance: Self::Place(place),
                                    },
                                ))
                            }
                            _ => place
                                .structure_field(identifier)
                                .map(|(place, access)| (Element::Place(place), access))
                                .map_err(Error::Place)
                                .map_err(SemanticError::Element),
                        },
                        _ => place
                            .structure_field(identifier)
                            .map(|(place, access)| (Element::Place(place), access))
                            .map_err(Error::Place)
                            .map_err(SemanticError::Element),
                    }
                }
                element => Err(SemanticError::Element(
                    Error::OperatorDotSecondOperandExpectedIdentifier {
                        location: element
                            .location()
                            .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                        found: element.to_string(),
                    },
                )),
            },
            Self::Value(value) => match other {
                Self::TupleIndex(index) => value
                    .tuple_field(index)
                    .map(|(value, access)| {
                        (Element::Value(value), DotAccessVariant::StackField(access))
                    })
                    .map_err(Error::Value)
                    .map_err(SemanticError::Element),
                Self::Identifier(identifier) => {
                    let scope = match value.r#type() {
                        Type::Structure(ref inner) => inner.scope.to_owned(),
                        Type::Enumeration(ref inner) => inner.scope.to_owned(),
                        Type::Contract(ref inner) => inner.scope.to_owned(),
                        _ => {
                            return value
                                .structure_field(identifier)
                                .map(|(value, access)| {
                                    (Element::Value(value), DotAccessVariant::StackField(access))
                                })
                                .map_err(Error::Value)
                                .map_err(SemanticError::Element)
                        }
                    };

                    let item = scope.borrow().resolve_item(&identifier, false);
                    match item {
                        Ok(item) => match *item.borrow() {
                            ScopeItem::Type(ref r#type) => {
                                let r#type = r#type.define()?;
                                Ok((
                                    Element::Type(r#type),
                                    DotAccessVariant::Method {
                                        instance: Self::Value(value),
                                    },
                                ))
                            }
                            _ => value
                                .structure_field(identifier)
                                .map(|(value, access)| {
                                    (Element::Value(value), DotAccessVariant::StackField(access))
                                })
                                .map_err(Error::Value)
                                .map_err(SemanticError::Element),
                        },
                        _ => value
                            .structure_field(identifier)
                            .map(|(value, access)| {
                                (Element::Value(value), DotAccessVariant::StackField(access))
                            })
                            .map_err(Error::Value)
                            .map_err(SemanticError::Element),
                    }
                }
                element => Err(SemanticError::Element(
                    Error::OperatorDotSecondOperandExpectedIdentifier {
                        location: element
                            .location()
                            .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                        found: element.to_string(),
                    },
                )),
            },
            Self::Constant(constant) => match other {
                Self::TupleIndex(index) => constant
                    .tuple_field(index)
                    .map(|(constant, access)| {
                        (
                            Element::Constant(constant),
                            DotAccessVariant::StackField(access),
                        )
                    })
                    .map_err(Error::Constant)
                    .map_err(SemanticError::Element),
                Self::Identifier(identifier) => {
                    let scope = match constant.r#type() {
                        Type::Structure(ref inner) => inner.scope.to_owned(),
                        Type::Enumeration(ref inner) => inner.scope.to_owned(),
                        Type::Contract(ref inner) => inner.scope.to_owned(),
                        _ => {
                            return constant
                                .structure_field(identifier)
                                .map(|(constant, access)| {
                                    (
                                        Element::Constant(constant),
                                        DotAccessVariant::StackField(access),
                                    )
                                })
                                .map_err(Error::Constant)
                                .map_err(SemanticError::Element)
                        }
                    };

                    let item = scope.borrow().resolve_item(&identifier, false);
                    match item {
                        Ok(item) => match *item.borrow() {
                            ScopeItem::Type(ref r#type) => {
                                let r#type = r#type.define()?;
                                Ok((
                                    Element::Type(r#type),
                                    DotAccessVariant::Method {
                                        instance: Self::Constant(constant),
                                    },
                                ))
                            }
                            _ => constant
                                .structure_field(identifier)
                                .map(|(constant, access)| {
                                    (
                                        Element::Constant(constant),
                                        DotAccessVariant::StackField(access),
                                    )
                                })
                                .map_err(Error::Constant)
                                .map_err(SemanticError::Element),
                        },
                        _ => constant
                            .structure_field(identifier)
                            .map(|(constant, access)| {
                                (
                                    Element::Constant(constant),
                                    DotAccessVariant::StackField(access),
                                )
                            })
                            .map_err(Error::Constant)
                            .map_err(SemanticError::Element),
                    }
                }
                element => Err(SemanticError::Element(
                    Error::OperatorDotSecondOperandExpectedIdentifier {
                        location: element
                            .location()
                            .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                        found: element.to_string(),
                    },
                )),
            },
            element => Err(SemanticError::Element(
                Error::OperatorDotFirstOperandExpectedPlaceOrEvaluable {
                    location: element
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element.to_string(),
                },
            )),
        }
    }

    pub fn path(self, other: Self) -> Result<Self, Error> {
        let mut path = match self {
            Self::Path(path) => path,
            element => {
                return Err(Error::OperatorPathFirstOperandExpectedPath {
                    location: element
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element.to_string(),
                })
            }
        };

        let identifier = match other {
            Self::Identifier(identifier) => identifier,
            element => {
                return Err(Error::OperatorPathSecondOperandExpectedIdentifier {
                    location: element
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element.to_string(),
                })
            }
        };

        path.push_element(identifier);

        Ok(Self::Path(path))
    }

    pub fn structure(self, other: Self) -> Result<Self, Error> {
        match self {
            Element::Type(Type::Structure(r#type)) => match other {
                Element::Value(Value::Structure(mut structure)) => {
                    structure
                        .validate(r#type)
                        .map_err(ValueError::Structure)
                        .map_err(Error::Value)?;

                    Ok(Self::Value(Value::Structure(structure)))
                }
                Element::Constant(Constant::Structure(mut structure)) => {
                    structure
                        .validate(r#type)
                        .map_err(ConstantError::Structure)
                        .map_err(Error::Constant)?;

                    Ok(Self::Constant(Constant::Structure(structure)))
                }
                element => Err(Error::OperatorStructureSecondOperandExpectedEvaluable {
                    location: element
                        .location()
                        .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    found: element.to_string(),
                }),
            },
            element => Err(Error::OperatorStructureFirstOperandExpectedType {
                location: element
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                found: element.to_string(),
            }),
        }
    }

    pub fn location(&self) -> Option<Location> {
        match self {
            Self::Value(inner) => inner.location(),
            Self::Constant(inner) => Some(inner.location()),
            Self::Type(inner) => inner.location(),
            Self::ArgumentList(inner) => Some(inner.location),
            Self::Path(inner) => Some(inner.location),
            Self::Place(inner) => Some(inner.identifier.location),
            Self::TupleIndex(inner) => Some(inner.location),
            Self::Identifier(inner) => Some(inner.location),
            Self::Module(inner) => Some(inner.location),
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Value(inner) => write!(f, "value {}", inner),
            Self::Constant(inner) => write!(f, "constant {}", inner),
            Self::Type(inner) => write!(f, "type {}", inner),
            Self::ArgumentList(inner) => write!(f, "argument list {}", inner),
            Self::Path(inner) => write!(f, "path {}", inner),
            Self::Place(inner) => write!(f, "place {}", inner),
            Self::TupleIndex(inner) => write!(f, "tuple field {}", inner),
            Self::Identifier(inner) => write!(f, "structure field {}", inner.name),
            Self::Module(inner) => write!(f, "module {}", inner.name),
        }
    }
}
