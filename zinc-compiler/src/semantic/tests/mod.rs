//!
//! A semantic analyzer test.
//!

#![allow(dead_code)]

mod error_conditional_branch_types_mismatch;
mod error_conditional_expected_boolean_condition;
mod error_const_expression_has_non_const_element;
mod error_impl_expected_structure_or_enumeration;
mod error_loop_bounds_expected_constant_range_expression;
mod error_loop_while_expected_boolean_condition;
mod error_match_branch_expression_invalid_type;
mod error_match_branch_pattern_invalid_type;
mod error_match_branch_pattern_path_expected_evaluable;
mod error_match_branch_unreachable;
mod error_match_not_exhausted;
mod error_module_not_found;
mod error_mutating_immutable_memory;
mod error_mutating_with_different_type;
mod error_structure_duplicate_field;
mod error_type_alias_does_not_point_to_structure;
mod error_type_alias_does_not_point_to_type;
mod error_use_expected_path;

pub static PANIC_TEST_DATA: &str = "Test data is always valid";

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use zinc_bytecode::Instruction;

use crate::error::Error;
use crate::Bytecode;
use crate::EntryPointAnalyzer;
use crate::ModuleAnalyzer;
use crate::Parser;
use crate::Scope;

static PANIC_SYNTAX_ERROR: &str = "Syntax errors must be eliminated at this point";
static PANIC_THE_ONLY_REFERENCE: &str =
    "The last shared reference is always unwrapped successfully";

pub fn compile_entry_point(input: &str) -> Result<(), Error> {
    EntryPointAnalyzer::default().compile(
        Parser::default()
            .parse(input, None)
            .expect(PANIC_SYNTAX_ERROR),
        HashMap::new(),
    )
}

pub(self) fn compile_module(
    input: &str,
    bytecode: Rc<RefCell<Bytecode>>,
) -> Result<Rc<RefCell<Scope>>, Error> {
    ModuleAnalyzer::new(bytecode).compile(
        Parser::default()
            .parse(input, None)
            .expect(PANIC_SYNTAX_ERROR),
    )
}

pub(self) fn get_instructions(input: &str) -> Result<Vec<Instruction>, Error> {
    get_instructions_with_dependencies(
        input,
        Rc::new(RefCell::new(Bytecode::new())),
        HashMap::new(),
    )
}

pub(self) fn get_instructions_with_dependencies(
    input: &str,
    bytecode: Rc<RefCell<Bytecode>>,
    dependencies: HashMap<String, Rc<RefCell<Scope>>>,
) -> Result<Vec<Instruction>, Error> {
    EntryPointAnalyzer::new(bytecode.clone()).compile(
        Parser::default()
            .parse(input, None)
            .expect(PANIC_SYNTAX_ERROR),
        dependencies,
    )?;
    let instructions: Vec<Instruction> = Rc::try_unwrap(bytecode)
        .expect(PANIC_THE_ONLY_REFERENCE)
        .into_inner()
        .into();
    Ok(instructions
        .into_iter()
        .filter_map(|instruction| match instruction {
            Instruction::FileMarker(_) => None,
            Instruction::FunctionMarker(_) => None,
            Instruction::LineMarker(_) => None,
            Instruction::ColumnMarker(_) => None,
            instruction => Some(instruction),
        })
        .collect::<Vec<Instruction>>())
}
