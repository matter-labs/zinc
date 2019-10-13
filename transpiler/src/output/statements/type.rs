//!
//! Transpiler output type statement.
//!

use parser::Identifier;
use parser::Type;

use crate::TypeOutput;

pub struct Output {}

impl Output {
    pub fn output(identifier: Identifier, r#type: Type) -> String {
        format!(
            "type {0} = {1};",
            identifier,
            TypeOutput::output(r#type.variant)
        )
    }
}
