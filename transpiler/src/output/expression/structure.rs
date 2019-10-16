//!
//! Transpiler output structure.
//!

use parser::Identifier;

use crate::element::Element;

pub struct Output {}

impl Output {
    pub fn output(
        identifier: String,
        type_name: String,
        fields: &[(Identifier, Element)],
    ) -> String {
        format!(
            "let {0} = {1} {{ {2} }};",
            identifier,
            type_name,
            fields
                .iter()
                .map(|(key, identifier)| format!("{}: {}", key, identifier))
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}
