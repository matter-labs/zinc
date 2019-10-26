//!
//! Transpiler output enum statement.
//!

use parser::Identifier;
use parser::Variant;

pub struct Output {}

impl Output {
    pub fn output(identifier: Identifier, variants: Vec<Variant>) -> String {
        format!(
            r#"enum {0} {{ {1} }}"#,
            identifier,
            variants
                .into_iter()
                .map(|variant| variant.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}
