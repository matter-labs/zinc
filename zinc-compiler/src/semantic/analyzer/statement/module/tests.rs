//!
//! The `mod` statement tests.
//!

#![cfg(test)]

use std::collections::HashMap;

use crate::source::module::Module as SourceModule;

#[test]
fn ok_multiple_module_constants_sum() {
    let module_1 = r#"
const A: u8 = 25;
"#;

    let module_2 = r#"
const B: u8 = 42;
"#;

    let module_3 = r#"
const C: u8 = 64;
"#;

    let binary = r#"
mod one;
mod two;
mod three;

fn main() -> u8 {
    one::A + two::B + three::C
}
"#;

    let module_1 = SourceModule::test(module_1, HashMap::new()).expect(crate::panic::TEST_DATA);
    let module_2 = SourceModule::test(module_2, HashMap::new()).expect(crate::panic::TEST_DATA);
    let module_3 = SourceModule::test(module_3, HashMap::new()).expect(crate::panic::TEST_DATA);

    let dependencies: HashMap<String, SourceModule> = vec![
        ("one".to_owned(), module_1),
        ("two".to_owned(), module_2),
        ("three".to_owned(), module_3),
    ]
    .into_iter()
    .collect();

    assert!(crate::semantic::tests::compile_entry_with_dependencies(binary, dependencies).is_ok());
}
