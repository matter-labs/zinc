//!
//! Transpiler output array.
//!

use crate::VariableOutput;

pub struct Output {
    pub identifier: String,
    pub elements: Vec<String>,
}

impl Output {
    pub fn new(identifier: String, elements: Vec<VariableOutput>) -> Self {
        Self {
            identifier,
            elements: elements
                .into_iter()
                .map(|element| element.into())
                .collect::<Vec<String>>(),
        }
    }
}

impl Into<String> for Output {
    fn into(self) -> String {
        format!(
            "let {0} = [{1}];",
            self.identifier,
            self.elements.join(", ")
        )
    }
}
