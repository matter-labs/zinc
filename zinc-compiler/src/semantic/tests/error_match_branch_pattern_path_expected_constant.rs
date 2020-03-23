//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Location;
use crate::semantic::element::r#type::Type;
use crate::semantic::scope::Scope;
use crate::semantic::Error as SemanticError;

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
        SemanticError::MatchBranchPatternPathExpectedConstant {
            location: Location::new(7, 9),
            found: Type::field().to_string(),
        },
    ));

    let module_1 = super::compile_module(module_1).expect(PANIC_COMPILE_DEPENDENCY);

    let dependencies: HashMap<String, Rc<RefCell<Scope>>> = vec![("module_1".to_owned(), module_1)]
        .into_iter()
        .collect();

    let result = super::compile_entry_point_with_dependencies(binary, dependencies);

    assert_eq!(result, expected);
}
