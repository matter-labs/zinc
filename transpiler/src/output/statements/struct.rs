//!
//! Transpiler output struct statement.
//!

use parser::Identifier;
use parser::Type;

use crate::TypeOutput;

pub struct Output {
    pub identifier: String,
    pub fields: Vec<(String, String)>,
}

impl Output {
    pub fn new(identifier: Identifier, fields: Vec<(Identifier, Type)>) -> Self {
        Self {
            identifier: identifier.name,
            fields: fields
                .into_iter()
                .map(|(identifier, r#type)| {
                    (identifier.name, TypeOutput::from(r#type.variant).into())
                })
                .collect::<Vec<(String, String)>>(),
        }
    }
}

impl Into<String> for Output {
    fn into(self) -> String {
        format!(
            r#"struct {0} {{ {1} }}"#,
            self.identifier,
            self.fields
                .into_iter()
                .map(|(identifier, r#type)| format!("{}: {}", identifier, r#type))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
