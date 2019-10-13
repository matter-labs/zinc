//!
//! Transpiler output assignment operator.
//!

use crate::Element;

pub struct Output {}

impl Output {
    pub fn output(operand_1: Element, operand_2: Element) -> String {
        format!(
            r#"{0} = {1};"#,
            operand_1,
            match operand_2 {
                Element::Permanent(element) => format!("{}.clone()", element),
                element => element.to_string(),
            }
        )
    }
}
