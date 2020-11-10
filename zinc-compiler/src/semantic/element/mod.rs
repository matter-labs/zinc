//!
//! The semantic analyzer element.
//!

#[cfg(test)]
mod tests;

pub mod access;
pub mod argument_list;
pub mod constant;
pub mod path;
pub mod place;
pub mod tuple_index;
pub mod r#type;
pub mod value;

use std::cell::RefCell;
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
use std::rc::Rc;

use zinc_lexical::Location;
use zinc_syntax::Identifier;

use crate::generator::expression::operator::Operator as GeneratorExpressionOperator;
use crate::semantic::element::r#type::i_typed::ITyped;
use crate::semantic::error::Error;
use crate::semantic::scope::item::Item as ScopeItem;
use crate::semantic::scope::Scope;

use self::access::dot::Dot as DotAccessVariant;
use self::access::index::Index as IndexAccess;
use self::argument_list::ArgumentList;
use self::constant::Constant;
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
    ///
    /// Executes the `=` assignment operator.
    ///
    pub fn assign(self, other: Self) -> Result<(Place, GeneratorExpressionOperator), Error> {
        match other {
            Self::Value(_) => {}
            Self::Constant(_) => {}
            element => {
                return Err(Error::OperatorAssignmentSecondOperandExpectedEvaluable {
                    location: element
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element.to_string(),
                })
            }
        }

        match self {
            Self::Place(place) => Ok((place, GeneratorExpressionOperator::None)),
            element => Err(Error::OperatorAssignmentFirstOperandExpectedPlace {
                location: element
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element.to_string(),
            }),
        }
    }

    ///
    /// Executes the `|=` assignment with bitwise OR operator.
    ///
    pub fn assign_bitor(self, other: Self) -> Result<(Place, GeneratorExpressionOperator), Error> {
        match self {
            Self::Place(place) => {
                let value_1 = Value::try_from_place(&place)?;
                match other {
                    Self::Value(value_2) => value_1
                        .bitor(value_2)
                        .map(|(_value, operator)| (place, operator)),
                    Self::Constant(value_2) => value_1
                        .bitor(Value::try_from_constant(value_2)?)
                        .map(|(_value, operator)| (place, operator)),
                    element => Err(
                        Error::OperatorAssignmentBitwiseOrSecondOperandExpectedEvaluable {
                            location: element
                                .location()
                                .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(
                Error::OperatorAssignmentBitwiseOrFirstOperandExpectedPlace {
                    location: element
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element.to_string(),
                },
            ),
        }
    }

    ///
    /// Executes the `^=` assignment with bitwise XOR operator.
    ///
    pub fn assign_bitxor(self, other: Self) -> Result<(Place, GeneratorExpressionOperator), Error> {
        match self {
            Self::Place(place) => {
                let value_1 = Value::try_from_place(&place)?;
                match other {
                    Self::Value(value_2) => value_1
                        .bitxor(value_2)
                        .map(|(_value, operator)| (place, operator)),
                    Self::Constant(value_2) => value_1
                        .bitxor(Value::try_from_constant(value_2)?)
                        .map(|(_value, operator)| (place, operator)),
                    element => Err(
                        Error::OperatorAssignmentBitwiseXorSecondOperandExpectedEvaluable {
                            location: element
                                .location()
                                .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(
                Error::OperatorAssignmentBitwiseXorFirstOperandExpectedPlace {
                    location: element
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element.to_string(),
                },
            ),
        }
    }

    ///
    /// Executes the `&=` assignment with bitwise AND operator.
    ///
    pub fn assign_bitand(self, other: Self) -> Result<(Place, GeneratorExpressionOperator), Error> {
        match self {
            Self::Place(place) => {
                let value_1 = Value::try_from_place(&place)?;
                match other {
                    Self::Value(value_2) => value_1
                        .bitand(value_2)
                        .map(|(_value, operator)| (place, operator)),
                    Self::Constant(value_2) => value_1
                        .bitand(Value::try_from_constant(value_2)?)
                        .map(|(_value, operator)| (place, operator)),
                    element => Err(
                        Error::OperatorAssignmentBitwiseAndSecondOperandExpectedEvaluable {
                            location: element
                                .location()
                                .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(
                Error::OperatorAssignmentBitwiseAndFirstOperandExpectedPlace {
                    location: element
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element.to_string(),
                },
            ),
        }
    }

    ///
    /// Executes the `<<=` assignment with bitwise shift left operator.
    ///
    pub fn assign_bitwise_shift_left(
        self,
        other: Self,
    ) -> Result<(Place, GeneratorExpressionOperator), Error> {
        match self {
            Self::Place(place) => {
                let value_1 = Value::try_from_place(&place)?;
                match other {
                    Self::Value(value_2) => value_1
                        .shl(value_2)
                        .map(|(_value, operator)| (place, operator)),
                    Self::Constant(value_2) => value_1
                        .shl(Value::try_from_constant(value_2)?)
                        .map(|(_value, operator)| (place, operator)),
                    element => Err(
                        Error::OperatorAssignmentBitwiseShiftLeftSecondOperandExpectedEvaluable {
                            location: element
                                .location()
                                .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(
                Error::OperatorAssignmentBitwiseShiftLeftFirstOperandExpectedPlace {
                    location: element
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element.to_string(),
                },
            ),
        }
    }

    ///
    /// Executes the `>>=` assignment with bitwise shift right operator.
    ///
    pub fn assign_bitwise_shift_right(
        self,
        other: Self,
    ) -> Result<(Place, GeneratorExpressionOperator), Error> {
        match self {
            Self::Place(place) => {
                let value_1 = Value::try_from_place(&place)?;
                match other {
                    Self::Value(value_2) => value_1
                        .shr(value_2)
                        .map(|(_value, operator)| (place, operator)),
                    Self::Constant(value_2) => value_1
                        .shr(Value::try_from_constant(value_2)?)
                        .map(|(_value, operator)| (place, operator)),
                    element => Err(
                        Error::OperatorAssignmentBitwiseShiftRightSecondOperandExpectedEvaluable {
                            location: element
                                .location()
                                .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(
                Error::OperatorAssignmentBitwiseShiftRightFirstOperandExpectedPlace {
                    location: element
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element.to_string(),
                },
            ),
        }
    }

    ///
    /// Executes the `+=` assignment with addition operator.
    ///
    pub fn assign_add(self, other: Self) -> Result<(Place, GeneratorExpressionOperator), Error> {
        match self {
            Self::Place(place) => {
                let value_1 = Value::try_from_place(&place)?;
                match other {
                    Self::Value(value_2) => value_1
                        .add(value_2)
                        .map(|(_value, operator)| (place, operator)),
                    Self::Constant(value_2) => value_1
                        .add(Value::try_from_constant(value_2)?)
                        .map(|(_value, operator)| (place, operator)),
                    element => Err(
                        Error::OperatorAssignmentAdditionSecondOperandExpectedEvaluable {
                            location: element
                                .location()
                                .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(Error::OperatorAssignmentAdditionFirstOperandExpectedPlace {
                location: element
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element.to_string(),
            }),
        }
    }

    ///
    /// Executes the `-=` assignment with subtraction operator.
    ///
    pub fn assign_subtract(
        self,
        other: Self,
    ) -> Result<(Place, GeneratorExpressionOperator), Error> {
        match self {
            Self::Place(place) => {
                let value_1 = Value::try_from_place(&place)?;
                match other {
                    Self::Value(value_2) => value_1
                        .sub(value_2)
                        .map(|(_value, operator)| (place, operator)),
                    Self::Constant(value_2) => value_1
                        .sub(Value::try_from_constant(value_2)?)
                        .map(|(_value, operator)| (place, operator)),
                    element => Err(
                        Error::OperatorAssignmentSubtractionSecondOperandExpectedEvaluable {
                            location: element
                                .location()
                                .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(
                Error::OperatorAssignmentSubtractionFirstOperandExpectedPlace {
                    location: element
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element.to_string(),
                },
            ),
        }
    }

    ///
    /// Executes the `*=` assignment with multiplication operator.
    ///
    pub fn assign_multiply(
        self,
        other: Self,
    ) -> Result<(Place, GeneratorExpressionOperator), Error> {
        match self {
            Self::Place(place) => {
                let value_1 = Value::try_from_place(&place)?;
                match other {
                    Self::Value(value_2) => value_1
                        .mul(value_2)
                        .map(|(_value, operator)| (place, operator)),
                    Self::Constant(value_2) => value_1
                        .mul(Value::try_from_constant(value_2)?)
                        .map(|(_value, operator)| (place, operator)),
                    element => Err(
                        Error::OperatorAssignmentMultiplicationSecondOperandExpectedEvaluable {
                            location: element
                                .location()
                                .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(
                Error::OperatorAssignmentMultiplicationFirstOperandExpectedPlace {
                    location: element
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element.to_string(),
                },
            ),
        }
    }

    ///
    /// Executes the `/=` assignment with division operator.
    ///
    pub fn assign_divide(self, other: Self) -> Result<(Place, GeneratorExpressionOperator), Error> {
        match self {
            Self::Place(place) => {
                let value_1 = Value::try_from_place(&place)?;
                match other {
                    Self::Value(value_2) => value_1
                        .div(value_2)
                        .map(|(_value, operator)| (place, operator)),
                    Self::Constant(value_2) => value_1
                        .div(Value::try_from_constant(value_2)?)
                        .map(|(_value, operator)| (place, operator)),
                    element => Err(
                        Error::OperatorAssignmentDivisionSecondOperandExpectedEvaluable {
                            location: element
                                .location()
                                .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(Error::OperatorAssignmentDivisionFirstOperandExpectedPlace {
                location: element
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element.to_string(),
            }),
        }
    }

    ///
    /// Executes the `%=` assignment with remainder operator.
    ///
    pub fn assign_remainder(
        self,
        other: Self,
    ) -> Result<(Place, GeneratorExpressionOperator), Error> {
        match self {
            Self::Place(place) => {
                let value_1 = Value::try_from_place(&place)?;
                match other {
                    Self::Value(value_2) => value_1
                        .rem(value_2)
                        .map(|(_value, operator)| (place, operator)),
                    Self::Constant(value_2) => value_1
                        .rem(Value::try_from_constant(value_2)?)
                        .map(|(_value, operator)| (place, operator)),
                    element => Err(
                        Error::OperatorAssignmentRemainderSecondOperandExpectedEvaluable {
                            location: element
                                .location()
                                .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                            found: element.to_string(),
                        },
                    ),
                }
            }
            element => Err(
                Error::OperatorAssignmentRemainderFirstOperandExpectedPlace {
                    location: element
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element.to_string(),
                },
            ),
        }
    }

    ///
    /// Executes the `..=` range inclusive operator.
    ///
    pub fn range_inclusive(self, other: Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Constant(value_1), Element::Constant(value_2)) => {
                value_1.range_inclusive(value_2).map(Self::Constant)
            }
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorRangeInclusiveSecondOperandExpectedConstant {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorRangeInclusiveFirstOperandExpectedConstant {
                location: element_1
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element_1.to_string(),
            }),
        }
    }

    ///
    /// Executes the `..` range operator.
    ///
    pub fn range(self, other: Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Constant(value_1), Element::Constant(value_2)) => {
                value_1.range(value_2).map(Self::Constant)
            }
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorRangeSecondOperandExpectedConstant {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorRangeFirstOperandExpectedConstant {
                location: element_1
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element_1.to_string(),
            }),
        }
    }

    ///
    /// Executes the `||` logical OR operator.
    ///
    pub fn or(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .or(value_2)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .or(Value::try_from_constant(value_2)?)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorOrSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)?
                    .or(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .or(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator)),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorOrSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorOrFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element_1.to_string(),
            }),
        }
    }

    ///
    /// Executes the `^^` logical XOR operator.
    ///
    pub fn xor(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .xor(value_2)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .xor(Value::try_from_constant(value_2)?)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorXorSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)?
                    .xor(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .xor(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator)),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorXorSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorXorFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element_1.to_string(),
            }),
        }
    }

    ///
    /// Executes the `&&` logical AND operator.
    ///
    pub fn and(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .and(value_2)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .and(Value::try_from_constant(value_2)?)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorAndSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)?
                    .and(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .and(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator)),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorAndSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorAndFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element_1.to_string(),
            }),
        }
    }

    ///
    /// Executes the `==` equals comparison operator.
    ///
    pub fn equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .equals(value_2)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .equals(Value::try_from_constant(value_2)?)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorEqualsSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)?
                    .equals(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .equals(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator)),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorEqualsSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorEqualsFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element_1.to_string(),
            }),
        }
    }

    ///
    /// Executes the `!=` not-equals comparison operator.
    ///
    pub fn not_equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .not_equals(value_2)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .not_equals(Value::try_from_constant(value_2)?)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorNotEqualsSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)?
                    .not_equals(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .not_equals(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator)),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorNotEqualsSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorNotEqualsFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element_1.to_string(),
            }),
        }
    }

    ///
    /// Executes the `>=` greater-equals comparison operator.
    ///
    pub fn greater_equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .greater_equals(value_2)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .greater_equals(Value::try_from_constant(value_2)?)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorGreaterEqualsSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)?
                    .greater_equals(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .greater_equals(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator)),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorGreaterEqualsSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorGreaterEqualsFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element_1.to_string(),
            }),
        }
    }

    ///
    /// Executes the `<=` lesser-equals comparison operator.
    ///
    pub fn lesser_equals(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .lesser_equals(value_2)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .lesser_equals(Value::try_from_constant(value_2)?)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorLesserEqualsSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)?
                    .lesser_equals(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .lesser_equals(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator)),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorLesserEqualsSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorLesserEqualsFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element_1.to_string(),
            }),
        }
    }

    ///
    /// Executes the `>` greater comparison operator.
    ///
    pub fn greater(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .greater(value_2)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .greater(Value::try_from_constant(value_2)?)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorGreaterSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)?
                    .greater(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .greater(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator)),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorGreaterSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorGreaterFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element_1.to_string(),
            }),
        }
    }

    ///
    /// Executes the `<` lesser comparison operator.
    ///
    pub fn lesser(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .lesser(value_2)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .lesser(Value::try_from_constant(value_2)?)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorLesserSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)?
                    .lesser(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .lesser(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator)),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorLesserSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorLesserFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element_1.to_string(),
            }),
        }
    }
}

impl BitOr for Element {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn bitor(self, other: Self) -> Self::Output {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .bitor(value_2)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .bitor(Value::try_from_constant(value_2)?)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorBitwiseOrSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)?
                    .bitor(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .bitor(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator)),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorBitwiseOrSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorBitwiseOrFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element_1.to_string(),
            }),
        }
    }
}

impl BitXor for Element {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn bitxor(self, other: Self) -> Self::Output {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .bitxor(value_2)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .bitxor(Value::try_from_constant(value_2)?)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorBitwiseXorSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)?
                    .bitxor(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .bitxor(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator)),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorBitwiseXorSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorBitwiseXorFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element_1.to_string(),
            }),
        }
    }
}

impl BitAnd for Element {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn bitand(self, other: Self) -> Self::Output {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .bitand(value_2)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .bitand(Value::try_from_constant(value_2)?)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorBitwiseAndSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)?
                    .bitand(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .bitand(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator)),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorBitwiseAndSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorBitwiseAndFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element_1.to_string(),
            }),
        }
    }
}

impl Shl<Self> for Element {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn shl(self, other: Self) -> Self::Output {
        match (self, other) {
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .shl(Value::try_from_constant(value_2)?)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(_), element_2) => Err(
                Error::OperatorBitwiseShiftLeftSecondOperandExpectedConstant {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                },
            ),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .shl(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator)),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorBitwiseShiftLeftSecondOperandExpectedConstant {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                },
            ),
            (element_1, _) => Err(
                Error::OperatorBitwiseShiftLeftFirstOperandExpectedEvaluable {
                    location: element_1
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_1.to_string(),
                },
            ),
        }
    }
}

impl Shr<Self> for Element {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn shr(self, other: Self) -> Self::Output {
        match (self, other) {
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .shr(Value::try_from_constant(value_2)?)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(_), element_2) => Err(
                Error::OperatorBitwiseShiftRightSecondOperandExpectedConstant {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                },
            ),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .shr(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator)),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorBitwiseShiftRightSecondOperandExpectedConstant {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                },
            ),
            (element_1, _) => Err(
                Error::OperatorBitwiseShiftRightFirstOperandExpectedEvaluable {
                    location: element_1
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_1.to_string(),
                },
            ),
        }
    }
}

impl Add for Element {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .add(value_2)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .add(Value::try_from_constant(value_2)?)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorAdditionSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)?
                    .add(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .add(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator)),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorAdditionSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorAdditionFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element_1.to_string(),
            }),
        }
    }
}

impl Sub for Element {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .sub(value_2)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .sub(Value::try_from_constant(value_2)?)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorSubtractionSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)?
                    .sub(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .sub(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator)),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorSubtractionSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorSubtractionFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element_1.to_string(),
            }),
        }
    }
}

impl Mul for Element {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .mul(value_2)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .mul(Value::try_from_constant(value_2)?)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(_), element_2) => Err(
                Error::OperatorMultiplicationSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                },
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)?
                    .mul(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .mul(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator)),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorMultiplicationSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                },
            ),
            (element_1, _) => Err(Error::OperatorMultiplicationFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element_1.to_string(),
            }),
        }
    }
}

impl Div for Element {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .div(value_2)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .div(Value::try_from_constant(value_2)?)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorDivisionSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)?
                    .div(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .div(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator)),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorDivisionSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorDivisionFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element_1.to_string(),
            }),
        }
    }
}

impl Rem for Element {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn rem(self, other: Self) -> Self::Output {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .rem(value_2)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .rem(Value::try_from_constant(value_2)?)
                .map(|(value, operator)| (Self::Value(value), operator)),
            (Element::Value(_), element_2) => {
                Err(Error::OperatorRemainderSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (Element::Constant(value_1), Element::Value(value_2)) => {
                Value::try_from_constant(value_1)?
                    .rem(value_2)
                    .map(|(value, operator)| (Self::Value(value), operator))
            }
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .rem(value_2)
                .map(|(constant, operator)| (Self::Constant(constant), operator)),
            (Element::Constant(_), element_2) => {
                Err(Error::OperatorRemainderSecondOperandExpectedEvaluable {
                    location: element_2
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element_2.to_string(),
                })
            }
            (element_1, _) => Err(Error::OperatorRemainderFirstOperandExpectedEvaluable {
                location: element_1
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element_1.to_string(),
            }),
        }
    }
}

impl Element {
    ///
    /// Executes the `as` casting operator.
    ///
    pub fn cast(self, other: Self) -> Result<(Self, Option<GeneratorExpressionOperator>), Error> {
        let r#type = match other {
            Self::Type(r#type) => r#type,
            element => {
                return Err(Error::OperatorCastingSecondOperandExpectedType {
                    location: element
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element.to_string(),
                })
            }
        };

        match self {
            Element::Value(value) => value
                .cast(r#type)
                .map(|(value, operator)| (Self::Value(value), operator)),
            Element::Constant(constant) => constant
                .cast(r#type)
                .map(|(constant, operator)| (Self::Constant(constant), operator)),
            element => Err(Error::OperatorCastingFirstOperandExpectedEvaluable {
                location: element
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element.to_string(),
            }),
        }
    }

    ///
    /// Executes the `!` logical NOT operator.
    ///
    pub fn not(self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Element::Value(value) => value
                .not()
                .map(|(value, operator)| (Self::Value(value), operator)),
            Element::Constant(constant) => constant
                .not()
                .map(|(constant, operator)| (Self::Constant(constant), operator)),
            element => Err(Error::OperatorNotExpectedEvaluable {
                location: element
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element.to_string(),
            }),
        }
    }

    ///
    /// Executes the `~` bitwise NOT operator.
    ///
    pub fn bitwise_not(self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        match self {
            Element::Value(value) => value
                .bitwise_not()
                .map(|(value, operator)| (Self::Value(value), operator)),
            Element::Constant(constant) => constant
                .bitwise_not()
                .map(|(constant, operator)| (Self::Constant(constant), operator)),
            element => Err(Error::OperatorBitwiseNotExpectedEvaluable {
                location: element
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element.to_string(),
            }),
        }
    }
}

impl Neg for Element {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn neg(self) -> Self::Output {
        match self {
            Element::Value(value) => value
                .neg()
                .map(|(value, operator)| (Self::Value(value), operator)),
            Element::Constant(constant) => constant
                .neg()
                .map(|(constant, operator)| (Self::Constant(constant), operator)),
            element => Err(Error::OperatorNegationExpectedEvaluable {
                location: element
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element.to_string(),
            }),
        }
    }
}

impl Element {
    ///
    /// Executes the `[]` array index operator.
    ///
    pub fn index(self, other: Self) -> Result<(Self, IndexAccess), Error> {
        match self {
            Self::Place(place) => match other {
                element @ Self::Value(_) => place
                    .index(element)
                    .map(|(place, access)| (Element::Place(place), access)),
                element @ Self::Constant(_) => place
                    .index(element)
                    .map(|(place, access)| (Element::Place(place), access)),
                element => Err(Error::OperatorIndexSecondOperandExpectedEvaluable {
                    location: element
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element.to_string(),
                }),
            },
            Self::Value(value) => match other {
                Self::Value(index) => value
                    .index_value(index)
                    .map(|(value, access)| (Element::Value(value), access)),
                Self::Constant(index) => value
                    .index_constant(index)
                    .map(|(value, access)| (Element::Value(value), access)),
                element => Err(Error::OperatorIndexSecondOperandExpectedEvaluable {
                    location: element
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element.to_string(),
                }),
            },
            Self::Constant(constant) => match other {
                Self::Value(index) => constant.index_value(index),
                Self::Constant(index) => constant.index_constant(index),
                element => Err(Error::OperatorIndexSecondOperandExpectedEvaluable {
                    location: element
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element.to_string(),
                }),
            },
            element => Err(Error::OperatorIndexFirstOperandExpectedPlaceOrEvaluable {
                location: element
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element.to_string(),
            }),
        }
    }

    ///
    /// Executes the `.` dot field access operator.
    ///
    pub fn dot(self, other: Self) -> Result<(Self, DotAccessVariant), Error> {
        match self {
            Self::Place(place) => match other {
                Self::TupleIndex(index) => place
                    .tuple_field(index)
                    .map(|(place, access)| (Element::Place(place), access)),
                Self::Identifier(identifier) => {
                    let scope = match place.r#type {
                        Type::Structure(ref inner) => inner.scope.to_owned(),
                        Type::Enumeration(ref inner) => inner.scope.to_owned(),
                        Type::Contract(ref inner) => inner.scope.to_owned(),
                        _ => {
                            return place
                                .structure_field(identifier)
                                .map(|(place, access)| (Element::Place(place), access))
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
                                        instance: Box::new(Self::Place(place)),
                                    },
                                ))
                            }
                            _ => place
                                .structure_field(identifier)
                                .map(|(place, access)| (Element::Place(place), access)),
                        },
                        _ => place
                            .structure_field(identifier)
                            .map(|(place, access)| (Element::Place(place), access)),
                    }
                }
                element => Err(Error::OperatorDotSecondOperandExpectedIdentifier {
                    location: element
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element.to_string(),
                }),
            },
            Self::Value(value) => match other {
                Self::TupleIndex(index) => value.tuple_field(index).map(|(value, access)| {
                    (Element::Value(value), DotAccessVariant::StackField(access))
                }),
                Self::Identifier(identifier) => {
                    let scope = match value.r#type() {
                        Type::Structure(ref inner) => inner.scope.to_owned(),
                        Type::Enumeration(ref inner) => inner.scope.to_owned(),
                        Type::Contract(ref inner) => inner.scope.to_owned(),
                        _ => {
                            return value
                                .structure_field(identifier)
                                .map(|(value, access)| (Element::Value(value), access))
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
                                        instance: Box::new(Self::Value(value)),
                                    },
                                ))
                            }
                            _ => value
                                .structure_field(identifier)
                                .map(|(value, access)| (Element::Value(value), access)),
                        },
                        _ => value
                            .structure_field(identifier)
                            .map(|(value, access)| (Element::Value(value), access)),
                    }
                }
                element => Err(Error::OperatorDotSecondOperandExpectedIdentifier {
                    location: element
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element.to_string(),
                }),
            },
            Self::Constant(constant) => match other {
                Self::TupleIndex(index) => constant.tuple_field(index).map(|(constant, access)| {
                    (
                        Element::Constant(constant),
                        DotAccessVariant::StackField(access),
                    )
                }),
                Self::Identifier(identifier) => {
                    let scope = match constant.r#type() {
                        Type::Structure(ref inner) => inner.scope.to_owned(),
                        Type::Enumeration(ref inner) => inner.scope.to_owned(),
                        Type::Contract(ref inner) => inner.scope.to_owned(),
                        _ => {
                            return constant.structure_field(identifier).map(
                                |(constant, access)| {
                                    (
                                        Element::Constant(constant),
                                        DotAccessVariant::StackField(access),
                                    )
                                },
                            )
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
                                        instance: Box::new(Self::Constant(constant)),
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
                                }),
                        },
                        _ => constant
                            .structure_field(identifier)
                            .map(|(constant, access)| {
                                (
                                    Element::Constant(constant),
                                    DotAccessVariant::StackField(access),
                                )
                            }),
                    }
                }
                element => Err(Error::OperatorDotSecondOperandExpectedIdentifier {
                    location: element
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element.to_string(),
                }),
            },
            element => Err(Error::OperatorDotFirstOperandExpectedPlaceOrEvaluable {
                location: element
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element.to_string(),
            }),
        }
    }

    ///
    /// Executes the `::` path operator.
    ///
    pub fn path(self, other: Self) -> Result<Self, Error> {
        let mut path = match self {
            Self::Path(path) => path,
            element => {
                return Err(Error::OperatorPathFirstOperandExpectedPath {
                    location: element
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
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
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element.to_string(),
                })
            }
        };

        path.push_element(identifier);

        Ok(Self::Path(path))
    }

    ///
    /// Executes the structure initialization operator.
    ///
    /// It is a special internal operator, which accepts the structure type and literal as operands.
    ///
    pub fn structure(self, other: Self, scope: Rc<RefCell<Scope>>) -> Result<Self, Error> {
        match self {
            Element::Type(Type::Structure(r#type)) => match other {
                Element::Value(Value::Structure(mut structure)) => {
                    structure.validate(r#type)?;

                    Ok(Self::Value(Value::Structure(structure)))
                }
                Element::Constant(Constant::Structure(mut structure)) => {
                    structure.validate(r#type)?;

                    Ok(Self::Constant(Constant::Structure(structure)))
                }
                element => Err(Error::OperatorStructureSecondOperandExpectedLiteral {
                    location: element
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element.to_string(),
                }),
            },
            Element::Type(Type::Contract(r#type)) => match other {
                Element::Value(Value::Structure(structure)) => {
                    let mut contract = structure.into_contract(scope);
                    contract.validate(r#type)?;

                    Ok(Self::Value(Value::Contract(contract)))
                }
                Element::Value(Value::Contract(mut contract)) => {
                    contract.validate(r#type)?;

                    Ok(Self::Value(Value::Contract(contract)))
                }
                element => Err(Error::OperatorStructureSecondOperandExpectedLiteral {
                    location: element
                        .location()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: element.to_string(),
                }),
            },
            element => Err(Error::OperatorStructureFirstOperandExpectedType {
                location: element
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: element.to_string(),
            }),
        }
    }

    ///
    /// The semantic element location in the source code.
    ///
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
