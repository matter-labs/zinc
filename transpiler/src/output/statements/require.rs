//!
//! Transpiler output require statement.
//!

use crate::Element;

pub struct Output {}

impl Output {
    pub fn output(expression: Element, annotation: String) -> String {
        format!(
            r#"r1cs::require(system.namespace(|| "{1}"), &{0}, "{1}");"#,
            expression, annotation
        )
    }
}
