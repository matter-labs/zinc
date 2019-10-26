//!
//! Transpiler output imports.
//!

use parser::Field;

use crate::output::InputOutput;
use crate::output::WitnessOutput;

pub struct Output {
    pub start: Vec<String>,
    pub end: Vec<String>,
}

impl Output {
    pub fn output(inputs: Vec<Field>, witnesses: Vec<Field>) -> Self {
        let mut start = Vec::with_capacity((inputs.len() + witnesses.len()) * 2 + 6);
        start.push("#[derive(Default)]".to_owned());
        start.push("pub struct GeneratedCircuit {".to_owned());
        for input in inputs.iter() {
            start.push(format!("    pub {0}: Fr,", input.identifier.name));
        }
        for witness in witnesses.iter() {
            start.push(format!("    pub {0}: Fr,", witness.identifier.name));
        }
        start.push("}".to_owned());
        start.push(String::new());
        start.push("impl Circuit<Bn256> for GeneratedCircuit {".to_owned());
        start.push("    fn synthesize<S: ConstraintSystem<Bn256>>(self, system: &mut S) -> Result<(), SynthesisError> {".to_owned());
        for input in inputs.into_iter() {
            start.push(format!("        {0}", InputOutput::output(input)));
        }
        for witness in witnesses.into_iter() {
            start.push(format!("        {0}", WitnessOutput::output(witness)));
        }

        let end = vec![
            "        Ok(())".to_owned(),
            "    }".to_owned(),
            "}".to_owned(),
        ];

        Self { start, end }
    }
}
