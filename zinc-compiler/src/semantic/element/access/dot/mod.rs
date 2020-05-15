//!
//! The semantic analyzer element dot access.
//!

pub mod contract_field;
pub mod stack_field;

use crate::semantic::element::Element;

use self::contract_field::ContractField;
use self::stack_field::StackField;

///
/// Tuple or structure field access, or namespace method access data.
///
pub enum Dot {
    /// Stack data field access via the dot `.` operator
    StackField(StackField),
    /// Contract storage field access via the dot `.` operator
    ContractField(ContractField),
    /// Method call via the dot `.` operator
    Method { instance: Element },
}
