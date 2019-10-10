//!
//! The transpiler writer.
//!

use parser::Input;
use parser::Witness;

pub struct Writer {
    buffer: String,
    id_sequence: usize,
    offset: usize,
    loop_stack: Vec<String>,
    conditional_stack: Vec<(String, bool)>,
}

impl Default for Writer {
    fn default() -> Self {
        Self {
            buffer: String::with_capacity(1_048_576),
            id_sequence: 0,
            offset: 0,
            loop_stack: Vec::with_capacity(16),
            conditional_stack: Vec::with_capacity(16),
        }
    }
}

impl Writer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&mut self) -> String {
        let result = self.buffer.clone();
        self.buffer.clear();
        result
    }

    pub fn write_attributes(&mut self) {
        self.write_line("#![allow(unused_imports)]".to_owned());
        self.write_line("#![allow(unused_variables)]".to_owned());
        self.write_new_line();
    }

    pub fn write_imports(&mut self) {
        self.write_line("use r1cs::ConstraintSystem;".to_owned());
        self.write_line("use r1cs::Circuit;".to_owned());
        self.write_line("use r1cs::SynthesisError;".to_owned());
        self.write_line("use r1cs::Bn256;".to_owned());
        self.write_line("use r1cs::Fr;".to_owned());
        self.write_line("use r1cs::AllocatedNum;".to_owned());
        self.write_line("use r1cs::Boolean;".to_owned());
        self.write_new_line();
    }

    pub fn write_circuit_declaration(&mut self, inputs: &[Input], witnesses: &[Witness]) {
        self.write_line("#[derive(Default)]".to_owned());
        self.write_line("pub struct GeneratedCircuit {".to_owned());
        self.shift_forward();
        for input in inputs.iter() {
            self.write_line(format!("pub {0}: Fr,", input.identifier.name));
        }
        for witness in witnesses.iter() {
            self.write_line(format!("pub {0}: Fr,", witness.identifier.name));
        }
        self.shift_backward();
        self.write_line("}".to_owned());
        self.write_new_line();
    }

    pub fn write_circuit_header(&mut self) {
        self.write_line("impl Circuit<Bn256> for GeneratedCircuit {".to_owned());
        self.shift_forward();
        self.write_line("fn synthesize<S: ConstraintSystem<Bn256>>(self, system: &mut S) -> Result<(), SynthesisError> {".to_owned());
        self.shift_forward();
    }

    pub fn write_allocate_input(&mut self, value: &str) {
        self.write_line(format!(
            r#"let {0} = r1cs::allocate_input(system.namespace(|| "{0}"), || Ok(self.{0}), 254)?.0;"#,
            value,
        ));
    }

    pub fn write_allocate_witness(&mut self, value: &str) {
        self.write_line(format!(
            r#"let {0} = r1cs::allocate_witness(system.namespace(|| "{0}"), || Ok(self.{0}), 254)?.0;"#,
            value,
        ));
    }

    pub fn write_allocate_boolean(&mut self, value: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = r1cs::allocate_boolean(system.namespace(|| {1}), {2})?;"#,
            id, namespace, value
        ));
        id
    }

    pub fn write_allocate_number_constant(&mut self, value: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = r1cs::allocate_number(system.namespace(|| {1}), "{2}")?;"#,
            id, namespace, value
        ));
        id
    }

    pub fn write_allocate_number_loop_index(&mut self, name: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        let value = format!("{}_index.to_string().as_str()", name);
        self.write_line(format!(
            r#"let {0} = r1cs::allocate_number(system.namespace(|| {1}), {2})?;"#,
            name, namespace, value,
        ));
        id
    }

    pub fn write_conditional(&mut self, a: &str, b: &str, cond: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = r1cs::conditional(system.namespace(|| {1}), &{2}, &{3}, &{4})?;"#,
            id, namespace, a, b, cond
        ));
        id
    }

    pub fn write_circuit_trailer(&mut self) {
        self.write_line("Ok(())".to_owned());
        self.shift_backward();
        self.write_line("}".to_owned());
        self.shift_backward();
        self.write_line("}".to_owned());
    }

    pub fn enter_block(&mut self) -> String {
        let (id, _namespace) = self.next_id_and_namespace();
        self.write_line(format!("let {0} = {{", id));
        self.shift_forward();
        id
    }

    pub fn exit_block(&mut self) {
        self.shift_backward();
        self.write_line("};".to_owned());
    }

    pub fn enter_loop(&mut self, index_name: &str, range_start: &str, range_end: &str) {
        self.write_line(format!(
            "for {0}_index in {1}..{2} {{",
            index_name, range_start, range_end
        ));
        self.shift_forward();
        self.loop_stack.push(index_name.to_owned());
    }

    pub fn exit_loop(&mut self) {
        self.loop_stack.pop();
        self.shift_backward();
        self.write_line("};".to_owned());
    }

    pub fn enter_conditional(&mut self, condition_name: &str, value: bool) -> String {
        let (id, _namespace) = self.next_id_and_namespace();
        self.write_line(format!("let {0} = {{", id));
        self.shift_forward();
        self.conditional_stack
            .push((condition_name.to_owned(), value));
        id
    }

    pub fn exit_conditional(&mut self) {
        self.conditional_stack.pop();
        self.shift_backward();
        self.write_line("};".to_owned());
    }

    pub fn write_line(&mut self, line: String) {
        self.write_offset();
        self.buffer.push_str(&line);
        self.write_new_line();
    }

    fn write_offset(&mut self) {
        self.buffer.push_str(&" ".repeat(self.offset));
    }

    fn write_new_line(&mut self) {
        self.buffer.push('\n');
    }

    fn next_id_and_namespace(&mut self) -> (String, String) {
        self.id_sequence += 1;
        let id = format!(r#"temp_{0:06}"#, self.id_sequence);
        let namespace = if self.loop_stack.is_empty() {
            format!(r#""temp_{0:06}""#, self.id_sequence)
        } else {
            let indexes = self
                .loop_stack
                .iter()
                .map(|index| format!("{}_index", index))
                .collect::<Vec<String>>()
                .join(", ");
            format!(
                r#"format!("temp_{0:06}_{{}}", {1})"#,
                self.id_sequence, indexes
            )
        };
        (id, namespace)
    }

    fn shift_forward(&mut self) {
        self.offset += 4;
    }

    fn shift_backward(&mut self) {
        self.offset -= 4;
    }
}
