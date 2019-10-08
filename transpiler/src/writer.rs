//!
//! The transpiler writer.
//!

use parser::Input;
use parser::StructStatement;
use parser::TypeStatement;
use parser::TypeVariant;
use parser::Witness;

use crate::Converter;

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

    pub fn write_require(&mut self, result: &str, annotation: &str) {
        self.write_line(format!(
            r#"r1cs::require(system.namespace(|| "{0}"), &{1}, "{0}");"#,
            annotation, result
        ));
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

    pub fn write_type(&mut self, data: TypeStatement) {
        self.write_line(format!(
            r#"type {0} = {1};"#,
            data.identifier,
            Converter::r#type(data.r#type.variant)
        ));
    }

    pub fn write_struct(&mut self, data: StructStatement) {
        let fields = data
            .fields
            .into_iter()
            .map(|(identifier, r#type)| {
                format!("{}: {}", identifier, Converter::r#type(r#type.variant))
            })
            .collect::<Vec<String>>()
            .join(", ");
        self.write_line(format!(r#"struct {0} {{ {1} }}"#, data.identifier, fields));
    }

    pub fn write_assignment(&mut self, operand_1: &str, operand_2: &str) {
        if self.conditional_stack.is_empty() {
            self.write_line(format!(r#"{0} = {1};"#, operand_1, operand_2,))
        } else {
            let conditions = self
                .conditional_stack
                .iter()
                .map(|(name, value)| {
                    format!(
                        r#"{0}{1}.get_value().expect("Always returns a value")"#,
                        if *value { "" } else { "!" },
                        name
                    )
                })
                .collect::<Vec<String>>()
                .join(" && ");
            self.write_line(format!(r#"if {0} {{"#, conditions));
            self.shift_forward();
            self.write_line(format!(r#"{0} = {1};"#, operand_1, operand_2,));
            self.shift_backward();
            self.write_line("}".to_owned());
        }
    }

    pub fn write_or(&mut self, operand_1: &str, operand_2: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = r1cs::or(system.namespace(|| {1}), &{2}, &{3})?;"#,
            id, namespace, operand_1, operand_2
        ));
        id
    }

    pub fn write_xor(&mut self, operand_1: &str, operand_2: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = r1cs::xor(system.namespace(|| {1}), &{2}, &{3})?;"#,
            id, namespace, operand_1, operand_2
        ));
        id
    }

    pub fn write_and(&mut self, operand_1: &str, operand_2: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = r1cs::and(system.namespace(|| {1}), &{2}, &{3})?;"#,
            id, namespace, operand_1, operand_2
        ));
        id
    }

    pub fn write_equals_boolean(&mut self, operand_1: &str, operand_2: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = r1cs::equals_boolean(system.namespace(|| {1}), &{2}, &{3}, 254)?;"#,
            id, namespace, operand_1, operand_2
        ));
        id
    }

    pub fn write_equals_number(&mut self, operand_1: &str, operand_2: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = r1cs::equals_number(system.namespace(|| {1}), &{2}, &{3}, 254)?;"#,
            id, namespace, operand_1, operand_2
        ));
        id
    }

    pub fn write_not_equals_boolean(&mut self, operand_1: &str, operand_2: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = r1cs::not_equals_boolean(system.namespace(|| {1}), &{2}, &{3}, 254)?;"#,
            id, namespace, operand_1, operand_2
        ));
        id
    }

    pub fn write_not_equals_number(&mut self, operand_1: &str, operand_2: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = r1cs::not_equals_number(system.namespace(|| {1}), &{2}, &{3}, 254)?;"#,
            id, namespace, operand_1, operand_2
        ));
        id
    }

    pub fn write_greater_equals(&mut self, operand_1: &str, operand_2: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = r1cs::greater_equals(system.namespace(|| {1}), &{2}, &{3}, 254)?;"#,
            id, namespace, operand_1, operand_2
        ));
        id
    }

    pub fn write_lesser_equals(&mut self, operand_1: &str, operand_2: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = r1cs::lesser_equals(system.namespace(|| {1}), &{2}, &{3}, 254)?;"#,
            id, namespace, operand_1, operand_2
        ));
        id
    }

    pub fn write_greater(&mut self, operand_1: &str, operand_2: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = r1cs::greater(system.namespace(|| {1}), &{2}, &{3}, 254)?;"#,
            id, namespace, operand_1, operand_2
        ));
        id
    }

    pub fn write_lesser(&mut self, operand_1: &str, operand_2: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = r1cs::lesser(system.namespace(|| {1}), &{2}, &{3}, 254)?;"#,
            id, namespace, operand_1, operand_2
        ));
        id
    }

    pub fn write_addition(&mut self, operand_1: &str, operand_2: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = r1cs::add(system.namespace(|| {1}), &{2}, &{3}, 254)?.0;"#,
            id, namespace, operand_1, operand_2
        ));
        id
    }

    pub fn write_subtraction(&mut self, operand_1: &str, operand_2: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = r1cs::subtract(system.namespace(|| {1}), &{2}, &{3}, 254)?.0;"#,
            id, namespace, operand_1, operand_2
        ));
        id
    }

    pub fn write_multiplication(&mut self, operand_1: &str, operand_2: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = r1cs::multiply(system.namespace(|| {1}), &{2}, &{3}, 254)?.0;"#,
            id, namespace, operand_1, operand_2
        ));
        id
    }

    pub fn write_casting(&mut self, operand_1: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = r1cs::cast(system.namespace(|| {1}), &{2}, 254)?;"#,
            id, namespace, operand_1
        ));
        id
    }

    pub fn write_negation(&mut self, operand_1: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = r1cs::negate(system.namespace(|| {1}), &{2}, 254)?.0;"#,
            id, namespace, operand_1
        ));
        id
    }

    pub fn write_not(&mut self, operand_1: &str) -> String {
        let (id, namespace) = self.next_id_and_namespace();
        self.write_line(format!(
            r#"let {0} = r1cs::not(system.namespace(|| {1}), &{2})?;"#,
            id, namespace, operand_1
        ));
        id
    }

    pub fn write_attributes(&mut self) {
        self.write_line("#![allow(unused_imports)]".to_owned());
        self.write_line("#![allow(unused_variables)]".to_owned());
        self.write_empty_line();
    }

    pub fn write_imports(&mut self) {
        self.write_line("use r1cs::ConstraintSystem;".to_owned());
        self.write_line("use r1cs::Circuit;".to_owned());
        self.write_line("use r1cs::SynthesisError;".to_owned());
        self.write_line("use r1cs::Bn256;".to_owned());
        self.write_line("use r1cs::Fr;".to_owned());
        self.write_line("use r1cs::AllocatedNum;".to_owned());
        self.write_line("use r1cs::Boolean;".to_owned());
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
        self.buffer.push_str(&" ".repeat(self.offset));
        self.buffer.push_str(&line);
        self.buffer.push('\n');
    }

    fn write_empty_line(&mut self) {
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
