//!
//! The semantic analyzer place path element.
//!

use std::fmt;

use num_bigint::BigInt;

use crate::generator::expression::Expression as GeneratorExpression;
use crate::semantic::element::access::dot::contract_field::ContractField as ContractFieldAccess;
use crate::semantic::element::access::dot::stack_field::StackField as StackFieldAccess;
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
    StackField {
        access: StackFieldAccess,
    },
    ContractField {
        access: ContractFieldAccess,
    },
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IndexExpression { .. } => write!(f, "[<runtime>]"),
            Self::IndexConstant { constant, .. } => write!(f, "[{}]", constant.value),
            Self::IndexRange { start, end, .. } => write!(f, "[{} .. {}]", start, end),
            Self::IndexRangeInclusive { start, end, .. } => write!(f, "[{} ..= {}]", start, end),
            Self::StackField { access } => write!(f, ".{}", access.position),
            Self::ContractField { access } => write!(f, ".{}", access.position),
        }
    }
}
