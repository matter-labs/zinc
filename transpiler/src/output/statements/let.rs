//!
//! Transpiler output let statement.
//!

use parser::Identifier;
use parser::Type;

use crate::TypeOutput;

pub struct Output {
    pub is_mutable: bool,
    pub identifier: String,
    pub r#type: Option<String>,
    pub expression: String,
}

impl Output {
    pub fn new(
        is_mutable: bool,
        identifier: Identifier,
        r#type: Option<Type>,
        expression: String,
    ) -> Self {
        Self {
            is_mutable,
            identifier: identifier.name,
            r#type: r#type.map(|r#type| TypeOutput::from(r#type.variant).into()),
            expression,
        }
    }
}

impl Into<String> for Output {
    fn into(self) -> String {
        format!(
            "let{0} {1}{2} = {3};",
            if self.is_mutable { " mut" } else { "" },
            self.identifier,
            if let Some(r#type) = self.r#type {
                format!(": {}", r#type)
            } else {
                "".to_owned()
            },
            self.expression,
        )
    }
}
