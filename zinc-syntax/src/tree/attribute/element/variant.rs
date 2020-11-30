//!
//! The attribute variant.
//!

use crate::tree::attribute::element::Element;
use crate::tree::literal::Literal;

///
/// The attribute.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    /// The value assigned with the `=` operator, e.g. `#[value = 0x42]`.
    Value(Literal),
    /// The nested attribute, e.g. `#[msg(sender = 0x0)]`.
    Nested(Vec<Element>),
}
