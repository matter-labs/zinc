//!
//! The semantic analyzer place path element.
//!

use std::fmt;

use num_bigint::BigInt;

use crate::generator::expression::Expression as GeneratorExpression;
use crate::semantic::element::access::dot::field::Field as FieldAccess;
use crate::semantic::element::access::index::Index as IndexAccess;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;

#[derive(Debug, Clone)]
pub enum Element {
    IndexExpression {
        expression: GeneratorExpression,
        access: IndexAccess,
    },
    IndexConstant {
        constant: IntegerConstant,
        access: IndexAccess,
    },
    IndexRange {
        start: BigInt,
        end: BigInt,
        access: IndexAccess,
    },
    IndexRangeInclusive {
        start: BigInt,
        end: BigInt,
        access: IndexAccess,
    },
    Field {
        access: FieldAccess,
    },
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IndexExpression { .. } => write!(f, "[<runtime>]"),
            Self::IndexConstant { constant, .. } => write!(f, "[{}]", constant.value),
            Self::IndexRange { start, end, .. } => write!(f, "[{} .. {}]", start, end),
            Self::IndexRangeInclusive { start, end, .. } => write!(f, "[{} ..= {}]", start, end),
            Self::Field { access } => write!(f, ".{}", access.position),
        }
    }
}
