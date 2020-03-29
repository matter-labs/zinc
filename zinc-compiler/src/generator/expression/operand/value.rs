//!
//! The generator expression value operand.
//!

use crate::generator::r#type::Type;

#[derive(Debug, Clone)]
pub struct Value {
    pub r#type: Type,
}

impl Value {
    pub fn new(r#type: Type) -> Self {
        Self { r#type }
    }
}
