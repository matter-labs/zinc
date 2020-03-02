//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::lexical::Location;
use crate::semantic::Error as SemanticError;
use crate::Bytecode;
use crate::Error;
use crate::Scope;

static PANIC_COMPILE_DEPENDENCY: &str = "Dependencies are compiled successfullt";

#[test]
fn test() {
    let module_1 = r#"
type X = field;
"#;

    let binary = r#"
mod module_1;

fn main() -> u8 {
    let value = 42;
    match value {
        module_1::X => 1,
        _ => 0,
    }
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::MatchBranchPatternPathExpectedEvaluable(
            Location::new(7, 9),
            "field".to_owned(),
        ),
    ));

    let bytecode = Rc::new(RefCell::new(Bytecode::new()));
    let module_1 =
        super::compile_module(module_1, bytecode.clone()).expect(PANIC_COMPILE_DEPENDENCY);

    let dependencies: HashMap<String, Rc<RefCell<Scope>>> = vec![("module_1".to_owned(), module_1)]
        .into_iter()
        .collect();

    let result = super::get_instructions_with_dependencies(binary, bytecode, dependencies);

    assert_eq!(result, expected);
}
