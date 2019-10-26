//!
//! Transpiler output struct statement.
//!

use parser::Field;
use parser::Identifier;

pub struct Output {}

impl Output {
    pub fn output(identifier: Identifier, fields: Vec<Field>) -> String {
        format!(
            r#"struct {0} {{ {1} }}"#,
            identifier,
            fields
                .into_iter()
                .map(|field| field.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}
