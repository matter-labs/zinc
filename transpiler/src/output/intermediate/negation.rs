//!
//! Transpiler output negation intermediate.
//!

use crate::VariableOutput;

pub struct Output {
    pub identifier: String,
    pub namespace: String,
    pub operand: String,
}

impl Output {
    pub fn new(identifier: String, namespace: String, operand: VariableOutput) -> Self {
        Self {
            identifier,
            namespace,
            operand: operand.into(),
        }
    }
}

impl Into<String> for Output {
    fn into(self) -> String {
        format!(
            r#"let {0} = r1cs::negate(system.namespace(|| {1}), &{2})?;"#,
            self.identifier, self.namespace, self.operand,
        )
    }
}
