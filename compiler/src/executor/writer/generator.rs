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
    id_sequence: usize,
    offset: usize,
    loop_stack: Vec<String>,
    conditional_stack: Vec<(String, bool)>,
}

impl Generator {
    pub fn new(path: PathBuf) -> Self {
        Self {
            file: File::create(path).expect("File creating error"),
            id_sequence: 0,
            offset: 0,
            loop_stack: Vec::with_capacity(16),
            conditional_stack: Vec::with_capacity(16),
        }
    }

    pub fn write_let(&mut self, is_mutable: bool, name: &str, result: &str) {
        self.write_line(format!(
            "let{0} {1} = {2};",
            if is_mutable { " mut" } else { "" },
            name,
            result,
        ));
    }

    pub fn write_debug(&mut self, result: &str) {
        self.write_line(format!(r#"dbg!({0}.get_value());"#, result));
    }

    pub fn write_require(&mut self, result: &str, annotation: &str) {
        self.write_line(format!(
            r#"jab::require(&mut cs, &{0}, "{1}");"#,
            result, annotation
        ));
    }

    pub fn write_assignment(&mut self, operand_1: &str, operand_2: &str) {
        if self.conditional_stack.is_empty() {
            self.write_line(format!(r#"{0} = {1};"#, operand_1, operand_2));
        } else {
            let conditions = self
                .conditional_stack
                .iter()
                .map(|(name, value)| {
                    format!(
                        r#"{0}{1}.get_value().unwrap()"#,
                        if *value { "" } else { "!" },
                        name
                    )
                })
                .collect::<Vec<String>>()
                .join(" && ");
            self.write_line(format!(r#"if {0} {{"#, conditions));
            self.shift_forward();
            self.write_line(format!(r#"{0} = {1};"#, operand_1, operand_2));
            self.shift_backward();
            self.write_line("}".to_owned());
        }
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

    pub fn write_casting(&mut self, operand_1: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = jab::casting(&mut cs, &{1}, {2}, 254)?;"#,
            id, operand_1, namespace
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
            self.write_line(format!("pub {0}: Fr,", input.identifier.name));
        }
        for witness in witnesses.iter() {
            self.write_line(format!("pub {0}: Fr,", witness.identifier.name));
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

    pub fn write_allocate_input(&mut self, input: Input) {
        let bitlength = match input.r#type.variant {
            TypeVariant::Void => panic!("Must be a numeric value"),
            TypeVariant::Bool => 1,
            TypeVariant::Int { bitlength } => bitlength,
            TypeVariant::Uint { bitlength } => bitlength,
            TypeVariant::Field => 254,
        };
        self.write_line(format!(
            r#"let {0} = jab::input_allocation(&mut cs, || Ok(self.{0}), "{0}", {1})?.0;"#,
            input.identifier.name, bitlength,
        ));
    }

    pub fn write_allocate_witness(&mut self, witness: Witness) {
        let bitlength = match witness.r#type.variant {
            TypeVariant::Void => panic!("Must be a numeric value"),
            TypeVariant::Bool => 1,
            TypeVariant::Int { bitlength } => bitlength,
            TypeVariant::Uint { bitlength } => bitlength,
            TypeVariant::Field => 254,
        };
        self.write_line(format!(
            r#"let {0} = jab::witness_allocation(&mut cs, || Ok(self.{0}), "{0}", {1})?.0;"#,
            witness.identifier.name, bitlength,
        ));
    }

    pub fn write_allocate_boolean(&mut self, value: &str) -> String {
        let (id, _namespace) = self.next_id_and_namespace();
        self.write_line(format!(r#"let {0} = Boolean::constant({1});"#, id, value));
        id
    }

    pub fn write_allocate_number_constant(&mut self, value: &str) -> String {
        let (id, _namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = jab::allocation(&mut cs, "{0}", "{1}")?;"#,
            id, value
        ));
        id
    }

    pub fn write_allocate_number_loop_index(&mut self, name: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = jab::allocation(&mut cs, {1}, {0}_index.to_string().as_str())?;"#,
            name, namespace
        ));
        id
    }

    pub fn write_conditional(&mut self, a: &str, b: &str, cond: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = jab::conditional(&mut cs, &{1}, &{2}, &{3}, {4})?;"#,
            id, a, b, cond, namespace
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
            format!(r#""temp_{0:06}""#, self.id_sequence)
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
