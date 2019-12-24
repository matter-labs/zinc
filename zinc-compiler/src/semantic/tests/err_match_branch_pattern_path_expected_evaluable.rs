//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use num_bigint::BigInt;

use zinc_bytecode::Add;
use zinc_bytecode::Call;
use zinc_bytecode::Cast;
use zinc_bytecode::Exit;
use zinc_bytecode::Instruction;
use zinc_bytecode::Load;
use zinc_bytecode::LoadGlobal;
use zinc_bytecode::Mul;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;
use zinc_bytecode::Store;
use zinc_bytecode::StoreGlobal;
use zinc_bytecode::Sub;

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
    match 42 {
        module_1::X => 1,
        _ => 0,
    }
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::MatchBranchPatternPathExpectedEvaluable(
            Location::new(6, 9),
            "field".to_owned(),
        ),
    ));

    let bytecode = Rc::new(RefCell::new(Bytecode::new()));
    let module_1 =
        super::get_dependency(module_1, bytecode.clone()).expect(PANIC_COMPILE_DEPENDENCY);

    let dependencies: HashMap<String, Rc<RefCell<Scope>>> = vec![("module_1".to_owned(), module_1)]
        .into_iter()
        .collect();

    let result = super::get_instructions_with_dependencies(binary, bytecode, dependencies);

    assert_eq!(expected, result);
}
