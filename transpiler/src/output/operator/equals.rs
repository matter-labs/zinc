//!
//! Transpiler output equals operator.
//!

use parser::TypeVariant;

use crate::element::Element;

pub struct Output {}

impl Output {
    pub fn output(
        identifier: String,
        namespace: String,
        operand_1: Element,
        operand_2: Element,
    ) -> String {
        match (operand_1.type_variant(), operand_2.type_variant()) {
            (TypeVariant::Boolean, TypeVariant::Boolean) => format!(
                r#"let {0} = r1cs::equals_boolean(system.namespace(|| {1}), &{2}, &{3}, 254)?;"#,
                identifier, namespace, operand_1, operand_2,
            ),
            (TypeVariant::Field, TypeVariant::Field) => format!(
                r#"let {0} = r1cs::equals_number(system.namespace(|| {1}), &{2}, &{3}, 254)?;"#,
                identifier, namespace, operand_1, operand_2,
            ),
            (type_1, type_2) => panic!("Got invalid types: {} and {}", type_1, type_2),
        }
    }
}
