//!
//! The semantic analyzer element dot access.
//!

pub mod field;

use crate::semantic::element::Element;

use self::field::Field;

///
/// Tuple or structure field access, or namespace method access data.
///
pub enum Dot {
    /// Field access via the dot `.` operator
    Field(Field),
    /// Method call via the dot `.` operator
    Method { instance: Element },
}
