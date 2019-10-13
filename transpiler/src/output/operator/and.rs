//!
//! Transpiler output AND operator.
//!

use crate::Element;

pub struct Output {}

impl Output {
    pub fn output(
        identifier: String,
        namespace: String,
        operand_1: Element,
        operand_2: Element,
    ) -> String {
        format!(
            r#"let {0} = r1cs::and(system.namespace(|| {1}), &{2}, &{3})?;"#,
            identifier, namespace, operand_1, operand_2,
        )
    }
}
