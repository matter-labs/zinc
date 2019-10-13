//!
//! Transpiler output conditional.
//!

use crate::Element;

pub struct Output {}

impl Output {
    pub fn output(
        identifier: String,
        namespace: String,
        a: Element,
        b: Element,
        condition: Element,
    ) -> String {
        format!(
            r#"let {0} = r1cs::conditional(system.namespace(|| {1}), &{2}, &{3}, &{4})?;"#,
            identifier, namespace, a, b, condition,
        )
    }
}
