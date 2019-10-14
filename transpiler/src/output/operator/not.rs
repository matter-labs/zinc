//!
//! Transpiler output NOT operator.
//!

use crate::element::Element;

pub struct Output {}

impl Output {
    pub fn output(identifier: String, namespace: String, operand: Element) -> String {
        format!(
            r#"let {0} = r1cs::not(system.namespace(|| {1}), &{2})?;"#,
            identifier, namespace, operand,
        )
    }
}
