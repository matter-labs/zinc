//!
//! The interpreter generator.
//!

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::syntax::Input;
use crate::syntax::TypeVariant;
use crate::syntax::Witness;

pub struct Generator {
    file: File,
    loop_stack: Vec<String>,
    id_sequence: usize,
    offset: usize,
}

impl Generator {
    pub fn new(path: PathBuf) -> Self {
        let file = File::create(path).expect("File creating error");
        let loop_stack = Vec::with_capacity(16);
        let id_sequence = 0;
        let offset = 0;

        Self {
            file,
            loop_stack,
            id_sequence,
            offset,
        }
    }

    pub fn write_let(&mut self, is_mutable: bool, lvalue: &str, rvalue: &str) {
        self.write_line(format!(
            "let{} {} = {}.clone();",
            if is_mutable { " mut" } else { "" },
            lvalue,
            rvalue,
        ));
    }

    pub fn write_debug(&mut self, rvalue: &str) {
        self.write_line(format!(r#"dbg!({}.get_variable());"#, rvalue));
    }

    pub fn write_require(&mut self, expression: &str, annotation: &str) {
        self.write_line(format!(
            r#"jab::require(&mut cs, &{0}, "{1}");"#,
            expression, annotation
        ));
    }

    pub fn write_assignment(&mut self, lvalue: &str, rvalue: &str) {
        self.write_line(format!(r#"{} = {}.clone();"#, lvalue, rvalue));
    }

    pub fn write_or(&mut self, operand_1: &str, operand_2: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = jab::or(&mut cs, &{1}, &{2}, {3})?;"#,
            id, operand_1, operand_2, namespace
        ));
        id
    }

    pub fn write_xor(&mut self, operand_1: &str, operand_2: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = jab::xor(&mut cs, &{1}, &{2}, {3})?;"#,
            id, operand_1, operand_2, namespace
        ));
        id
    }

    pub fn write_and(&mut self, operand_1: &str, operand_2: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = jab::and(&mut cs, &{1}, &{2}, {3})?;"#,
            id, operand_1, operand_2, namespace
        ));
        id
    }

    pub fn write_equals(&mut self, operand_1: &str, operand_2: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = jab::equals(&mut cs, &{1}, &{2}, {3}, 254)?;"#,
            id, operand_1, operand_2, namespace
        ));
        id
    }

    pub fn write_not_equals(&mut self, operand_1: &str, operand_2: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = jab::not_equals(&mut cs, &{1}, &{2}, {3}, 254)?;"#,
            id, operand_1, operand_2, namespace
        ));
        id
    }

    pub fn write_greater_equals(&mut self, operand_1: &str, operand_2: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = jab::greater_equals(&mut cs, &{1}, &{2}, {3}, 254)?;"#,
            id, operand_1, operand_2, namespace
        ));
        id
    }

    pub fn write_lesser_equals(&mut self, operand_1: &str, operand_2: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = jab::lesser_equals(&mut cs, &{1}, &{2}, {3}, 254)?;"#,
            id, operand_1, operand_2, namespace
        ));
        id
    }

    pub fn write_greater(&mut self, operand_1: &str, operand_2: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = jab::greater(&mut cs, &{1}, &{2}, {3}, 254)?;"#,
            id, operand_1, operand_2, namespace
        ));
        id
    }

    pub fn write_lesser(&mut self, operand_1: &str, operand_2: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = jab::lesser(&mut cs, &{1}, &{2}, {3}, 254)?;"#,
            id, operand_1, operand_2, namespace
        ));
        id
    }

    pub fn write_addition(&mut self, operand_1: &str, operand_2: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = jab::addition(&mut cs, &{1}, &{2}, {3}, 254)?.0;"#,
            id, operand_1, operand_2, namespace
        ));
        id
    }

    pub fn write_subtraction(&mut self, operand_1: &str, operand_2: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = jab::subtraction(&mut cs, &{1}, &{2}, {3}, 254)?.0;"#,
            id, operand_1, operand_2, namespace
        ));
        id
    }

    pub fn write_multiplication(&mut self, operand_1: &str, operand_2: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = jab::multiplication(&mut cs, &{1}, &{2}, {3}, 254)?.0;"#,
            id, operand_1, operand_2, namespace
        ));
        id
    }

    pub fn write_casting(&mut self, rvalue: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = jab::casting(&mut cs, &{1}, {2}, 254)?;"#,
            id, rvalue, namespace
        ));
        id
    }

    pub fn write_negation(&mut self, operand_1: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = jab::negation(&mut cs, &{1}, {2}, 254)?.0;"#,
            id, operand_1, namespace
        ));
        id
    }

    pub fn write_not(&mut self, operand_1: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = jab::not(&mut cs, &{1}, {2})?;"#,
            id, operand_1, namespace
        ));
        id
    }

    pub fn write_imports(&mut self) {
        self.write_line("#![allow(unused_imports)]".to_owned());
        self.write_empty_line();
        self.write_line("use bellman::Circuit;".to_owned());
        self.write_line("use bellman::ConstraintSystem;".to_owned());
        self.write_line("use bellman::SynthesisError;".to_owned());
        self.write_line("use franklin_crypto::circuit::boolean::Boolean;".to_owned());
        self.write_line("use pairing::bn256::Bn256;".to_owned());
        self.write_line("use pairing::bn256::Fr;".to_owned());
        self.write_empty_line();
    }

    pub fn write_circuit_declaration(&mut self, inputs: &[Input], witnesses: &[Witness]) {
        self.write_line("#[derive(Default)]".to_owned());
        self.write_line("pub struct GeneratedCircuit {".to_owned());
        self.shift_forward();
        for input in inputs.iter() {
            self.write_line(format!("pub {}: Fr,", input.identifier().name()));
        }
        for witness in witnesses.iter() {
            self.write_line(format!("pub {}: Fr,", witness.identifier().name()));
        }
        self.shift_backward();
        self.write_line("}".to_owned());
        self.write_empty_line();
    }

    pub fn write_circuit_header(&mut self) {
        self.write_line("impl Circuit<Bn256> for GeneratedCircuit {".to_owned());
        self.shift_forward();
        self.write_line("fn synthesize<CS: ConstraintSystem<Bn256>>(self, mut cs: &mut CS) -> Result<(), SynthesisError> {".to_owned());
        self.shift_forward();
    }

    pub fn write_allocate_input(&mut self, input: &Input) {
        let bitlength = match input.r#type().variant() {
            TypeVariant::Void => panic!("Must be a numeric value"),
            TypeVariant::Bool => 1,
            TypeVariant::Int { bitlength } => bitlength,
            TypeVariant::Uint { bitlength } => bitlength,
            TypeVariant::Field => 254,
        };
        self.write_line(format!(
            r#"let {0} = jab::input_allocation(&mut cs, || Ok(self.{0}), "{0}", {1})?.0;"#,
            input.identifier().name(),
            bitlength,
        ));
    }

    pub fn write_allocate_witness(&mut self, witness: &Witness) {
        let bitlength = match witness.r#type().variant() {
            TypeVariant::Void => panic!("Must be a numeric value"),
            TypeVariant::Bool => 1,
            TypeVariant::Int { bitlength } => bitlength,
            TypeVariant::Uint { bitlength } => bitlength,
            TypeVariant::Field => 254,
        };
        self.write_line(format!(
            r#"let {0} = jab::witness_allocation(&mut cs, || Ok(self.{0}), "{0}", {1})?.0;"#,
            witness.identifier().name(),
            bitlength,
        ));
    }

    pub fn write_allocate_boolean(&mut self, rvalue: &str) -> String {
        let (id, _namespace) = self.next_id_and_namespace();
        self.write_line(format!(r#"let {0} = Boolean::constant({1});"#, id, rvalue));
        id
    }

    pub fn write_allocate_number_constant(&mut self, rvalue: &str) -> String {
        let (id, _namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = jab::allocation(&mut cs, "{0}", "{1}")?;"#,
            id, rvalue
        ));
        id
    }

    pub fn write_allocate_number_loop_index(&mut self, lvalue: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {} = jab::allocation(&mut cs, {}, {}_index.to_string().as_str())?;"#,
            lvalue, namespace, lvalue
        ));
        id
    }

    pub fn write_identifier(&mut self, name: &str) {
        self.write_line(name.to_owned());
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
        self.write_line(format!("let {} = {{", id));
        self.shift_forward();
        id
    }

    pub fn exit_block(&mut self) {
        self.shift_backward();
        self.write_line("};".to_owned());
    }

    pub fn enter_loop(&mut self, index_name: &str, range_start: &str, range_end: &str) {
        self.write_line(format!(
            "for {}_index in {}..{} {{",
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

    fn write_line(&mut self, line: String) {
        let mut data = Vec::with_capacity(self.offset + line.len() + 1);
        data.append(&mut vec![b' '; self.offset]);
        data.append(&mut line.into_bytes());
        data.push(b'\n');
        self.file
            .write_all(data.as_slice())
            .expect("Generator writing error");
    }

    fn write_empty_line(&mut self) {
        self.file.write_all(b"\n").expect("Generator writing error");
    }

    fn next_id_and_namespace(&mut self) -> (String, String) {
        self.id_sequence += 1;
        let id = format!(r#"temp_{0:06}"#, self.id_sequence);
        let namespace = if self.loop_stack.is_empty() {
            format!(r#""temp_{:06}""#, self.id_sequence)
        } else {
            let indexes = self
                .loop_stack
                .iter()
                .map(|index| format!("{}_index", index))
                .collect::<Vec<String>>()
                .join(", ");
            format!(
                r#"&format!("temp_{0:06}_{{}}", {1})"#,
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
