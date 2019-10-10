//!
//! Transpiler output AND intermediate.
//!

use crate::VariableOutput;

pub struct Output {
    pub identifier: String,
    pub namespace: String,
    pub operand_1: String,
    pub operand_2: String,
}

impl Output {
    pub fn new(
        identifier: String,
        namespace: String,
        operand_1: VariableOutput,
        operand_2: VariableOutput,
    ) -> Self {
        Self {
            identifier,
            namespace,
            operand_1: operand_1.into(),
            operand_2: operand_2.into(),
        }
    }
}

impl Into<String> for Output {
    fn into(self) -> String {
        format!(
            r#"let {0} = r1cs::and(system.namespace(|| {1}), &{2}, &{3})?;"#,
            self.identifier, self.namespace, self.operand_1, self.operand_2
        )
    }
}
