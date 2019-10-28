//!
//! Transpiler output imports.
//!

pub struct Output {}

impl Output {
    pub fn output() -> Vec<String> {
        vec![
            "use r1cs::ConstraintSystem;".to_owned(),
            "use r1cs::Circuit;".to_owned(),
            "use r1cs::SynthesisError;".to_owned(),
            "use r1cs::Bn256;".to_owned(),
            "use r1cs::Fr;".to_owned(),
            "use r1cs::Boolean;".to_owned(),
            "use r1cs::AllocatedNum;".to_owned(),
            String::new(),
        ]
    }
}
