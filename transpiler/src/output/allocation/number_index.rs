//!
//! Transpiler output loop index allocation.
//!

pub struct Output {}

impl Output {
    pub fn output(identifier: String, namespace: String) -> String {
        format!(
            r#"let {0} = r1cs::allocate_number(system.namespace(|| {1}), {0}_index.to_string().as_str())?;"#,
            identifier, namespace,
        )
    }
}
