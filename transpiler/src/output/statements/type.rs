//!
//! Transpiler output type statement.
//!

use parser::Identifier;
use parser::Type;

use crate::TypeOutput;

pub struct Output {
    pub identifier: String,
    pub r#type: String,
}

impl Output {
    pub fn new(identifier: Identifier, r#type: Type) -> Self {
        Self {
            identifier: identifier.name,
            r#type: TypeOutput::from(r#type.variant).into(),
        }
    }
}

impl Into<String> for Output {
    fn into(self) -> String {
        format!("type {0} = {1};", self.identifier, self.r#type)
    }
}
