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
    offset: usize,
}

impl Generator {
    pub fn new(path: PathBuf) -> Self {
        let file = File::create(path).expect("File creating error");
        let offset = 8;
        Self { file, offset }
    }

    pub fn increase_offset(&mut self) {
        self.offset += 4;
    }

    pub fn decrease_offset(&mut self) {
        self.offset -= 4;
    }

    pub fn write_let(&mut self, is_mutable: bool, lvalue: &str, rvalue: &str) {
        let string = format!(
            "let{} {} = {};",
            if is_mutable { " mut" } else { "" },
            lvalue,
            rvalue,
        );
        self.write_shifted_line(&string);
    }

    pub fn write_debug(&mut self, rvalue: &str) {
        let string = format!(r#"dbg!({}.get_variable());"#, rvalue);
        self.write_shifted_line(&string);
    }

    pub fn write_require(&mut self, expression: &str, name: &str) {
        let string = format!(r#"jab::require(&mut cs, &{}, "{}");"#, expression, name);
        self.write_shifted_line(&string);
    }

    pub fn write_expression(&mut self, lvalue: &str, rvalue: &str) {
        let string = format!(
            r#"let {} = {};"#,
            lvalue, rvalue
        );
        self.write_shifted_line(&string);
    }

    pub fn write_assignment(&mut self, lvalue: &str, rvalue: &str) {
        let string = format!(
            r#"{} = {};"#,
            lvalue, rvalue
        );
        self.write_shifted_line(&string);
    }

    pub fn write_addition(&mut self, lvalue: &str, operand_1: &str, operand_2: &str) {
        let string = format!(
            r#"let {} = jab::addition(&mut cs, &{}, &{}, "{}", 254)?.0;"#,
            lvalue, operand_1, operand_2, lvalue
        );
        self.write_shifted_line(&string);
    }

    pub fn write_subtraction(&mut self, lvalue: &str, operand_1: &str, operand_2: &str) {
        let string = format!(
            r#"let {} = jab::subtraction(&mut cs, &{}, &{}, "{}", 254)?.0;"#,
            lvalue, operand_1, operand_2, lvalue
        );
        self.write_shifted_line(&string);
    }

    pub fn write_multiplication(&mut self, lvalue: &str, operand_1: &str, operand_2: &str) {
        let string = format!(
            r#"let {} = jab::multiplication(&mut cs, &{}, &{}, "{}", 254)?.0;"#,
            lvalue, operand_1, operand_2, lvalue
        );
        self.write_shifted_line(&string);
    }

    pub fn write_equals(&mut self, lvalue: &str, operand_1: &str, operand_2: &str) {
        let string = format!(
            r#"let {} = jab::equals(&mut cs, &{}, &{}, "{}", 254)?;"#,
            lvalue, operand_1, operand_2, lvalue
        );
        self.write_shifted_line(&string);
    }

    pub fn write_not_equals(&mut self, lvalue: &str, operand_1: &str, operand_2: &str) {
        let string = format!(
            r#"let {} = jab::not_equals(&mut cs, &{}, &{}, "{}", 254)?;"#,
            lvalue, operand_1, operand_2, lvalue
        );
        self.write_shifted_line(&string);
    }

    pub fn write_greater_equals(&mut self, lvalue: &str, operand_1: &str, operand_2: &str) {
        let string = format!(
            r#"let {} = jab::greater_equals(&mut cs, &{}, &{}, "{}", 254)?;"#,
            lvalue, operand_1, operand_2, lvalue
        );
        self.write_shifted_line(&string);
    }

    pub fn write_lesser_equals(&mut self, lvalue: &str, operand_1: &str, operand_2: &str) {
        let string = format!(
            r#"let {} = jab::lesser_equals(&mut cs, &{}, &{}, "{}", 254)?;"#,
            lvalue, operand_1, operand_2, lvalue
        );
        self.write_shifted_line(&string);
    }

    pub fn write_greater(&mut self, lvalue: &str, operand_1: &str, operand_2: &str) {
        let string = format!(
            r#"let {} = jab::greater(&mut cs, &{}, &{}, "{}", 254)?;"#,
            lvalue, operand_1, operand_2, lvalue
        );
        self.write_shifted_line(&string);
    }

    pub fn write_lesser(&mut self, lvalue: &str, operand_1: &str, operand_2: &str) {
        let string = format!(
            r#"let {} = jab::lesser(&mut cs, &{}, &{}, "{}", 254)?;"#,
            lvalue, operand_1, operand_2, lvalue
        );
        self.write_shifted_line(&string);
    }

    pub fn write_negation(&mut self, lvalue: &str, operand_1: &str) {
        let string = format!(
            r#"let {} = jab::negation(&mut cs, &{}, "{}", 254)?.0;"#,
            lvalue, operand_1, lvalue
        );
        self.write_shifted_line(&string);
    }

    pub fn write_not(&mut self, lvalue: &str, operand_1: &str) {
        let string = format!(
            r#"let {} = jab::not(&mut cs, &{}, "{}")?;"#,
            lvalue, operand_1, lvalue
        );
        self.write_shifted_line(&string);
    }

    pub fn write_imports(&mut self) {
        self.write_line("use bellman::Circuit;");
        self.write_line("use bellman::ConstraintSystem;");
        self.write_line("use bellman::SynthesisError;");
        self.write_line("use ff::Field;");
        self.write_line("use ff::PrimeField;");
        self.write_line("use pairing::bn256::Bn256;");
        self.write_line("use pairing::bn256::Fr;");
        self.write_line("use franklin_crypto::circuit::num::AllocatedNum;");
        self.write_empty_line();
    }

    pub fn write_circuit_declaration(&mut self) {
        self.write_line("#[derive(Default)]");
        self.write_line("pub struct GeneratedCircuit {}");
    }

    pub fn write_synthesize_header(&mut self) {
        self.write_line("impl Circuit<Bn256> for GeneratedCircuit {");
        self.write_line("    fn synthesize<CS: ConstraintSystem<Bn256>>(self, mut cs: &mut CS) -> Result<(), SynthesisError> {");
    }

    pub fn write_allocate_input(&mut self, input: &Input) {
        let bitlength = match input.r#type().variant() {
            TypeVariant::Void => panic!("Witness must be a numeric value"),
            TypeVariant::Bool => 1,
            TypeVariant::Int { bitlength } => bitlength,
            TypeVariant::Uint { bitlength } => bitlength,
            TypeVariant::Field => 254,
        };
        let string = format!(
            r#"let {} = jab::alloc_input(&mut cs, || Ok(Fr::zero()), "{}", {})?.0;"#,
            input.identifier().name(),
            input.identifier().name(),
            bitlength,
        );
        self.write_shifted_line(&string);
    }

    pub fn write_allocate_witness(&mut self, witness: &Witness) {
        let bitlength = match witness.r#type().variant() {
            TypeVariant::Void => panic!("Witness must be a numeric value"),
            TypeVariant::Bool => 1,
            TypeVariant::Int { bitlength } => bitlength,
            TypeVariant::Uint { bitlength } => bitlength,
            TypeVariant::Field => 254,
        };
        let string = format!(
            r#"let {} = jab::alloc_witness(&mut cs, || Ok(Fr::zero()), "{}", {})?.0;"#,
            witness.identifier().name(),
            witness.identifier().name(),
            bitlength,
        );
        self.write_shifted_line(&string);
    }

    pub fn write_allocate(&mut self, lvalue: &str, rvalue: &str) {
        let string = format!(r#"let {} = AllocatedNum::alloc(cs.namespace(|| "{}"), || Ok(Fr::from_str("{}").unwrap()))?;"#, lvalue, lvalue, rvalue);
        self.write_shifted_line(&string);
    }

    pub fn write_synthesize_trailer(&mut self) {
        self.write_line("        Ok(())");
        self.write_line("    }");
        self.write_line("}");
    }

    pub fn write_line(&mut self, line: &str) {
        self.file
            .write_all(format!("{}\r\n", line).as_bytes())
            .unwrap();
    }

    pub fn write_shifted_line(&mut self, line: &str) {
        self.file.write_all(&vec![b' '; self.offset]).unwrap();
        self.file
            .write_all(format!("{}\r\n", line).as_bytes())
            .unwrap();
    }

    pub fn write_empty_line(&mut self) {
        self.file.write_all(b"\r\n").unwrap();
    }
}
