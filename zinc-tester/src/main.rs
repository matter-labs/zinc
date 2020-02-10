//!
//! The Zinc tester binary.
//!

use std::fs::File;
use std::io::Read;

use serde_derive::Deserialize;

use zinc_bytecode::data::values::Value;
use zinc_bytecode::program::Program;

#[derive(Debug, Deserialize)]
struct TestData {
    input: Value,
    output: Value,
}

fn main() {
    let mut file = File::open("zinc-tester/test.zn").unwrap();
    let size = file.metadata().unwrap().len() as usize;
    let mut string = String::with_capacity(size);
    file.read_to_string(&mut string).unwrap();
    println!("{}", string);

    let json = string
        .lines()
        .filter_map(|line| {
            if line.starts_with("//#") {
                Some(&line[3..])
            } else {
                None
            }
        })
        .collect::<Vec<&str>>()
        .join("");
    println!("{}", json);

    let bytecode = zinc_compiler::compile_test(string).unwrap();
    let bytecode: Vec<u8> = bytecode.into();
    let program = Program::from_bytes(bytecode.as_slice()).unwrap();
    let json = serde_json::from_str(&json).unwrap();
    let input = Value::from_typed_json(&json, &program.input).unwrap();

//    let output = zinc_vm::run::<Bn256>(&program, &input).unwrap();
//
//    println!("{:?}", test_data);
}
