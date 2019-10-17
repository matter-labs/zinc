//!
//! Transpiler output enum statement.
//!

use parser::Identifier;
use parser::IntegerLiteral;

pub struct Output {}

impl Output {
    pub fn output(identifier: Identifier, variants: Vec<(Identifier, IntegerLiteral)>) -> String {
        format!(
            r#"enum {0} {{ {1} }}"#,
            identifier,
            variants
                .into_iter()
                .map(|(identifier, value)| {
                    let value: usize = value.into();
                    format!("{} = {}", identifier.name, value)
                })
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}
