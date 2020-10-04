//!
//! The semantic analyzer place path element.
//!

use std::fmt;

use num::BigInt;

use crate::generator::expression::Expression as GeneratorExpression;
use crate::semantic::element::access::dot::contract_field::ContractField as ContractFieldAccess;
use crate::semantic::element::access::dot::stack_field::StackField as StackFieldAccess;
use crate::semantic::element::access::index::Index as IndexAccess;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;

///
/// The semantic analyzer memory place path parts.
///
#[derive(Debug, Clone)]
pub enum Element {
    /// Array indexing with a non-constant index, which is the second operand of the `[]` index operator.
    IndexExpression {
        /// The expression intermediate representation, which must be translated to the target code separately.
        expression: GeneratorExpression,
        /// The array access data, which helps to generate the target code without redundant calculations.
        access: IndexAccess,
    },
    /// Array indexing with a constant index, which is the second operand of the `[]` index operator.
    IndexConstant {
        /// The constant array index.
        constant: IntegerConstant,
        /// The array access data, which helps to generate the target code without redundant calculations.
        access: IndexAccess,
    },
    /// Array slicing with a constant range, which is the second operand of the `[]` index operator.
    IndexRange {
        /// The constant array slice range left bound.
        start: BigInt,
        /// The constant array slice range right bound.
        end: BigInt,
        /// The array access data, which helps to generate the target code without redundant calculations.
        access: IndexAccess,
    },
    /// Array slicing with an inclusive constant range, which is the second operand of the `[]` index operator.
    IndexRangeInclusive {
        /// The constant array slice range left bound.
        start: BigInt,
        /// The constant array slice range right bound.
        end: BigInt,
        /// The array access data, which helps to generate the target code without redundant calculations.
        access: IndexAccess,
    },
    /// The tuple or structure field access via the `.` operator.
    StackField {
        /// The data stack access data, which helps to generate the target code without redundant calculations.
        access: StackFieldAccess,
    },
    /// The contract storage field access via the `.` operator.
    ContractField {
        /// The contract storage access data, which helps to generate the target code without redundant calculations.
        access: ContractFieldAccess,
    },
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IndexExpression { .. } => write!(f, "[<runtime>]"),
            Self::IndexConstant { constant, .. } => write!(f, "[{}]", constant.value),
            Self::IndexRange { start, end, .. } => write!(f, "[{} .. {}]", start, end),
            Self::IndexRangeInclusive { start, end, .. } => write!(f, "[{} ..= {}]", start, end),
            Self::StackField { access } => write!(f, ".{}", access.name),
            Self::ContractField { access } => write!(f, ".{}", access.name),
        }
    }
}
