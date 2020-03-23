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
mod error_match_branch_pattern_path_expected_constant;
mod error_match_branch_unreachable;
mod error_match_less_than_two_branches;
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

use crate::error::Error;
use crate::EntryPointAnalyzer;
use crate::ModuleAnalyzer;
use crate::Parser;
use crate::Scope;

static PANIC_SYNTAX_ERROR: &str = "Syntax errors must be eliminated at this point";
static PANIC_THE_ONLY_REFERENCE: &str =
    "The last shared reference is always unwrapped successfully";

pub fn compile_entry_point(input: &str) -> Result<(), Error> {
    compile_entry_point_with_dependencies(input, HashMap::new())
}

pub fn compile_entry_point_with_dependencies(
    input: &str,
    dependencies: HashMap<String, Rc<RefCell<Scope>>>,
) -> Result<(), Error> {
    let _representation = EntryPointAnalyzer::default().compile(
        Parser::default()
            .parse(input, None)
            .expect(PANIC_SYNTAX_ERROR),
        dependencies,
    )?;
    Ok(())
}

pub(self) fn compile_module(input: &str) -> Result<Rc<RefCell<Scope>>, Error> {
    let (scope, _representation) = ModuleAnalyzer::new().compile(
        Parser::default()
            .parse(input, None)
            .expect(PANIC_SYNTAX_ERROR),
    )?;
    Ok(scope)
}
