//!
//! Transpiler output structure.
//!

use parser::Identifier;

use crate::VariableOutput;

pub struct Output {
    pub identifier: String,
    pub type_name: String,
    pub fields: Vec<(String, String)>,
}

impl Output {
    pub fn new(
        identifier: String,
        type_name: String,
        fields: Vec<(Identifier, VariableOutput)>,
    ) -> Self {
        Self {
            identifier,
            type_name,
            fields: fields
                .into_iter()
                .map(|(key, identifier)| (key.name, identifier.identifier))
                .collect::<Vec<(String, String)>>(),
        }
    }
}

impl Into<String> for Output {
    fn into(self) -> String {
        format!(
            "let {0} = {1} {{ {2} }};",
            self.identifier,
            self.type_name,
            self.fields
                .into_iter()
                .map(|(key, identifier)| format!("{0}: {1}", key, identifier))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
