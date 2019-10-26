//!
//! Transpiler output input.
//!

use parser::Field;

pub struct Output {}

impl Output {
    pub fn output(data: Field) -> String {
        format!(r#"let {0} = r1cs::allocate_input(system.namespace(|| "input_{0}"), || Ok(self.{0}), 254)?.0;"#, data.identifier.name)
    }
}
