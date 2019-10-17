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
                r#"let {0} = r1cs::equals_boolean(system.namespace(|| {1}), &{2}, &{3})?;"#,
                identifier, namespace, operand_1, operand_2
            ),
            (
                TypeVariant::IntegerUnsigned { bitlength: b1 },
                TypeVariant::IntegerUnsigned { bitlength: b2 },
            ) => {
                if b1 == b2 {
                    format!(
                        r#"let {0} = r1cs::equals_number(system.namespace(|| {1}), &{2}, &{3}, {4})?;"#,
                        identifier, namespace, operand_1, operand_2, b1,
                    )
                } else {
                    panic!("Got invalid bitlengths: {} and {}", b1, b2)
                }
            }
            (
                TypeVariant::IntegerSigned { bitlength: b1 },
                TypeVariant::IntegerSigned { bitlength: b2 },
            ) => {
                if b1 == b2 {
                    format!(
                        r#"let {0} = r1cs::equals_number(system.namespace(|| {1}), &{2}, &{3}, {4})?;"#,
                        identifier, namespace, operand_1, operand_2, b1,
                    )
                } else {
                    panic!("Got invalid bitlengths: {} and {}", b1, b2)
                }
            }
            (TypeVariant::Field, TypeVariant::Field) => format!(
                r#"let {0} = r1cs::equals_number(system.namespace(|| {1}), &{2}, &{3}, {4})?;"#,
                identifier,
                namespace,
                operand_1,
                operand_2,
                semantic::BITLENGTH_FIELD
            ),
            (type_1, type_2) => panic!("Got invalid types: {} and {}", type_1, type_2),
        }
    }
}
