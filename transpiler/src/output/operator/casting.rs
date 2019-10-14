//!
//! Transpiler output casting operator.
//!

use crate::element::Element;

pub struct Output {}

impl Output {
    pub fn output(
        identifier: String,
        namespace: String,
        operand: Element,
        _type: Element,
    ) -> String {
        format!(
            r#"let {0} = r1cs::cast(system.namespace(|| {1}), &{2}, 254)?;"#,
            identifier, namespace, operand,
        )
    }
}
