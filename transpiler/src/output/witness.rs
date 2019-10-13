//!
//! Transpiler output witness.
//!

use parser::Witness;

pub struct Output {}

impl Output {
    pub fn output(data: Witness) -> String {
        format!(r#"let {0} = r1cs::allocate_witness(system.namespace(|| "witness_{0}"), || Ok(self.{0}), 254)?.0;"#, data.identifier.name)
    }
}
