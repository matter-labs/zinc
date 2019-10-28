//!
//! Transpiler output constant number allocation.
//!

pub struct Output {}

impl Output {
    pub fn output(identifier: String, namespace: String, value: String) -> String {
        format!(
            r#"let {0} = r1cs::allocate_number(system.namespace(|| {1}), "{2}")?;"#,
            identifier, namespace, value,
        )
    }
}
