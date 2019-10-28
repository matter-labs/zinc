//!
//! Transpiler output struct statement.
//!

use parser::Identifier;
use parser::Type;

use crate::output::TypeOutput;

pub struct Output {}

impl Output {
    pub fn output(identifier: Identifier, fields: Vec<(Identifier, Type)>) -> String {
        format!(
            r#"struct {0} {{ {1} }}"#,
            identifier,
            fields
                .into_iter()
                .map(|(identifier, r#type)| format!(
                    "{}: {}",
                    identifier.name,
                    TypeOutput::output(r#type.variant)
                ))
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}
