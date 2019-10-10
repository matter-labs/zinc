//!
//! Transpiler output assignment intermediate.
//!

use crate::VariableOutput;

pub struct Output {
    pub operand_1: String,
    pub operand_2: String,
    pub to_clone: bool,
}

impl Output {
    pub fn new(operand_1: VariableOutput, operand_2: VariableOutput) -> Self {
        let to_clone = !operand_2.is_temporary;

        Self {
            operand_1: operand_1.into(),
            operand_2: operand_2.into(),
            to_clone,
        }
    }
}

impl Into<String> for Output {
    fn into(self) -> String {
        format!(
            r#"{0} = {1}{2};"#,
            self.operand_1,
            self.operand_2,
            if self.to_clone { ".clone()" } else { "" },
        )
    }
}

//pub fn write_assignment(&mut self, operand_1: &str, operand_2: &str) {
//    if self.conditional_stack.is_empty() {
//        self.write_line(format!(r#"{0} = {1};"#, operand_1, operand_2,))
//    } else {
//        let conditions = self
//            .conditional_stack
//            .iter()
//            .map(|(name, value)| {
//                format!(
//                    r#"{0}{1}.get_value().expect("Always returns a value")"#,
//                    if *value { "" } else { "!" },
//                    name
//                )
//            })
//            .collect::<Vec<String>>()
//            .join(" && ");
//        self.write_line(format!(r#"if {0} {{"#, conditions));
//        self.shift_forward();
//        self.write_line(format!(r#"{0} = {1};"#, operand_1, operand_2,));
//        self.shift_backward();
//        self.write_line("}".to_owned());
//    }
//}
