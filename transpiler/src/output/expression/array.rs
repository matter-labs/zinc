//!
//! Transpiler output array.
//!

use crate::Element;

pub struct Output {}

impl Output {
    pub fn output(identifier: String, elements: Vec<Element>) -> String {
        format!(
            "let {0} = [{1}];",
            identifier,
            elements
                .into_iter()
                .map(|element| element.into())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
