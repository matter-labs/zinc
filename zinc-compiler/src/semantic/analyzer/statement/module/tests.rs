//!
//! The `mod` statement tests.
//!

#![cfg(test)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::analyzer::statement::error::Error as StatementError;
use crate::semantic::analyzer::statement::module::error::Error as ModStatementError;
use crate::semantic::error::Error as SemanticError;
use crate::semantic::scope::Scope;

#[test]
fn ok_multiple_module_constants_sum() {
    let module_1 = r#"
const A: u8 = 25;
"#;

    let module_2 = r#"
const B: u8 = 42;
"#;

    let module_3 = r#"
const C: u8 = 69;
"#;

    let binary = r#"
mod one;
mod two;
mod three;

fn main() -> u8 {
    one::A + two::B + three::C
}
"#;

    let module_1 = crate::semantic::tests::compile_module(module_1).expect(crate::panic::TEST_DATA);
    let module_2 = crate::semantic::tests::compile_module(module_2).expect(crate::panic::TEST_DATA);
    let module_3 = crate::semantic::tests::compile_module(module_3).expect(crate::panic::TEST_DATA);

    let dependencies: HashMap<String, Rc<RefCell<Scope>>> = vec![
        ("one".to_owned(), module_1),
        ("two".to_owned(), module_2),
        ("three".to_owned(), module_3),
    ]
    .into_iter()
    .collect();

    let result = crate::semantic::tests::compile_entry_with_dependencies(binary, dependencies);

    assert!(result.is_ok());
}

#[test]
fn error_not_found() {
    let input = r#"
mod unknown;

fn main() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::Statement(
        StatementError::Mod(ModStatementError::NotFound {
            location: Location::new(2, 5),
            name: "unknown".to_owned(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
