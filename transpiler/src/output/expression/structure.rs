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
        fields: Vec<(Identifier, Element)>,
    ) -> String {
        format!(
            "let {0} = {1} {{ {2} }};",
            identifier,
            type_name,
            fields
                .into_iter()
                .map(|(key, identifier)| format!("{}: {}", key, identifier))
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}
