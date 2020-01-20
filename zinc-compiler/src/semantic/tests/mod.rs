//!
//! A semantic analyzer test.
//!

#![allow(dead_code)]

mod err_assignment_to_immutable_memory;
mod err_assignment_types_mismatch;
mod err_caster_from_invalid_type;
mod err_caster_to_invalid_type;
mod err_caster_to_lesser_bitlength;
mod err_conditional_branch_types_mismatch;
mod err_conditional_expected_boolean_condition;
mod err_const_expression_has_non_const_element;
mod err_element_addition_1st_expected_evaluable;
mod err_element_addition_2nd_expected_evaluable;
mod err_element_and_1st_expected_evaluable;
mod err_element_and_2nd_expected_evaluable;
mod err_element_assignment_1st_expected_place;
mod err_element_assignment_2nd_expected_evaluable;
mod err_element_casting_1st_expected_evaluable;
mod err_element_constant_addition_1st_expected_integer;
mod err_element_constant_addition_2nd_expected_integer;
mod err_element_constant_and_1st_expected_boolean;
mod err_element_constant_and_2nd_expected_boolean;
mod err_element_constant_casting_const;
mod err_element_constant_casting_static;
mod err_element_constant_division_1st_expected_integer;
mod err_element_constant_division_2nd_expected_integer;
mod err_element_constant_equals_1st_expected_primitive;
mod err_element_constant_equals_2nd_expected_boolean;
mod err_element_constant_equals_2nd_expected_integer;
mod err_element_constant_equals_2nd_expected_unit;
mod err_element_constant_greater_1st_expected_integer;
mod err_element_constant_greater_2nd_expected_integer;
mod err_element_constant_greater_equals_1st_expected_integer;
mod err_element_constant_greater_equals_2nd_expected_integer;
mod err_element_constant_integer_division_zero;
mod err_element_constant_integer_negation_too_big;
mod err_element_constant_integer_remainder_zero;
mod err_element_constant_integer_types_mismatch_addition;
mod err_element_constant_integer_types_mismatch_division;
mod err_element_constant_integer_types_mismatch_greater;
mod err_element_constant_integer_types_mismatch_greater_equals;
mod err_element_constant_integer_types_mismatch_lesser;
mod err_element_constant_integer_types_mismatch_lesser_equals;
mod err_element_constant_integer_types_mismatch_multiplication;
mod err_element_constant_integer_types_mismatch_remainder;
mod err_element_constant_integer_types_mismatch_subtraction;
mod err_element_constant_lesser_1st_expected_integer;
mod err_element_constant_lesser_2nd_expected_integer;
mod err_element_constant_lesser_equals_1st_expected_integer;
mod err_element_constant_lesser_equals_2nd_expected_integer;
mod err_element_constant_multiplication_1st_expected_integer;
mod err_element_constant_multiplication_2nd_expected_integer;
mod err_element_constant_negation_expected_integer;
mod err_element_constant_not_equals_1st_expected_primitive;
mod err_element_constant_not_equals_2nd_expected_boolean;
mod err_element_constant_not_equals_2nd_expected_integer;
mod err_element_constant_not_equals_2nd_expected_unit;
mod err_element_constant_not_expected_boolean;
mod err_element_constant_or_1st_expected_boolean;
mod err_element_constant_or_2nd_expected_boolean;
mod err_element_constant_remainder_1st_expected_integer;
mod err_element_constant_remainder_2nd_expected_integer;
mod err_element_constant_subtraction_1st_expected_integer;
mod err_element_constant_subtraction_2nd_expected_integer;
mod err_element_constant_xor_1st_expected_boolean;
mod err_element_constant_xor_2nd_expected_boolean;
mod err_element_division_1st_expected_evaluable;
mod err_element_division_2nd_expected_evaluable;
mod err_element_equals_1st_expected_evaluable;
mod err_element_equals_2nd_expected_evaluable;
mod err_element_field_1st_expected_place_or_evaluable;
mod err_element_greater_1st_expected_evaluable;
mod err_element_greater_2nd_expected_evaluable;
mod err_element_greater_equals_1st_expected_evaluable;
mod err_element_greater_equals_2nd_expected_evaluable;
mod err_element_index_1st_expected_place_or_evaluable;
mod err_element_index_2nd_expected_evaluable;
mod err_element_lesser_1st_expected_evaluable;
mod err_element_lesser_2nd_expected_evaluable;
mod err_element_lesser_equals_1st_expected_evaluable;
mod err_element_lesser_equals_2nd_expected_evaluable;
mod err_element_multiplication_1st_expected_evaluable;
mod err_element_multiplication_2nd_expected_evaluable;
mod err_element_negation_expected_evaluable;
mod err_element_not_equals_1st_expected_evaluable;
mod err_element_not_equals_2nd_expected_evaluable;
mod err_element_not_expected_evaluable;
mod err_element_or_1st_expected_evaluable;
mod err_element_or_2nd_expected_evaluable;
mod err_element_path_1st_expected_path;
mod err_element_path_2nd_expected_member_string;
mod err_element_place_field_1st_expected_structure;
mod err_element_place_field_1st_expected_tuple;
mod err_element_place_field_does_not_exist_in_structure;
mod err_element_place_field_does_not_exist_in_tuple;
mod err_element_place_index_1st_expected_array;
mod err_element_place_index_2nd_expected_integer;
mod err_element_remainder_1st_expected_evaluable;
mod err_element_remainder_2nd_expected_evaluable;
mod err_element_subtraction_1st_expected_evaluable;
mod err_element_subtraction_2nd_expected_evaluable;
mod err_element_value_addition_1st_expected_integer;
mod err_element_value_addition_2nd_expected_integer;
mod err_element_value_and_1st_expected_boolean;
mod err_element_value_and_2nd_expected_boolean;
mod err_element_value_array_invalid_type;
mod err_element_value_casting_let;
mod err_element_value_division_1st_expected_integer;
mod err_element_value_division_2nd_expected_integer;
mod err_element_value_equals_1st_expected_primitive;
mod err_element_value_equals_2nd_expected_boolean;
mod err_element_value_equals_2nd_expected_integer;
mod err_element_value_equals_2nd_expected_unit;
mod err_element_value_field_1st_expected_structure;
mod err_element_value_field_1st_expected_tuple;
mod err_element_value_greater_1st_expected_integer;
mod err_element_value_greater_2nd_expected_integer;
mod err_element_value_greater_equals_1st_expected_integer;
mod err_element_value_greater_equals_2nd_expected_integer;
mod err_element_value_index_1st_expected_array;
mod err_element_value_index_2nd_expected_integer;
mod err_element_value_integer_field_negation;
mod err_element_value_integer_types_mismatch_addition;
mod err_element_value_integer_types_mismatch_division;
mod err_element_value_integer_types_mismatch_equals;
mod err_element_value_integer_types_mismatch_greater;
mod err_element_value_integer_types_mismatch_greater_equals;
mod err_element_value_integer_types_mismatch_lesser;
mod err_element_value_integer_types_mismatch_lesser_equals;
mod err_element_value_integer_types_mismatch_multiplication;
mod err_element_value_integer_types_mismatch_not_equals;
mod err_element_value_integer_types_mismatch_remainder;
mod err_element_value_integer_types_mismatch_subtraction;
mod err_element_value_lesser_1st_expected_integer;
mod err_element_value_lesser_2nd_expected_integer;
mod err_element_value_lesser_equals_1st_expected_integer;
mod err_element_value_lesser_equals_2nd_expected_integer;
mod err_element_value_multiplication_1st_expected_integer;
mod err_element_value_multiplication_2nd_expected_integer;
mod err_element_value_negation_expected_integer;
mod err_element_value_not_equals_1st_expected_primitive;
mod err_element_value_not_equals_2nd_expected_boolean;
mod err_element_value_not_equals_2nd_expected_integer;
mod err_element_value_not_equals_2nd_expected_unit;
mod err_element_value_not_expected_boolean;
mod err_element_value_or_1st_expected_boolean;
mod err_element_value_or_2nd_expected_boolean;
mod err_element_value_remainder_1st_expected_integer;
mod err_element_value_remainder_2nd_expected_integer;
mod err_element_value_structure_field_already_exists;
mod err_element_value_structure_field_does_not_exist;
mod err_element_value_subtraction_1st_expected_integer;
mod err_element_value_subtraction_2nd_expected_integer;
mod err_element_value_tuple_field_does_not_exist;
mod err_element_value_xor_1st_expected_boolean;
mod err_element_value_xor_2nd_expected_boolean;
mod err_element_xor_1st_expected_evaluable;
mod err_element_xor_2nd_expected_evaluable;
mod err_function_argument_count_mismatch;
mod err_function_argument_type_mismatch;
mod err_function_calling_not_callable;
mod err_function_main_expected_input_as_first_argument;
mod err_function_main_expected_two_arguments;
mod err_function_main_expected_witness_as_second_argument;
mod err_function_main_missing;
mod err_function_not_instruction;
mod err_function_return_type_mismatch;
mod err_impl_expected_structure_or_enumeration;
mod err_inference_constant;
mod err_inference_constant_loop_bounds;
mod err_inference_constant_pattern_match;
mod err_instruction_debug_expected_string;
mod err_literal_structure_field_does_not_exist;
mod err_loop_range_end_expected_constant_integer_expression;
mod err_loop_range_start_expected_constant_integer_expression;
mod err_loop_while_expected_boolean_condition;
mod err_match_branch_expression_invalid_type;
mod err_match_branch_pattern_invalid_type;
mod err_match_branch_pattern_path_expected_evaluable;
mod err_match_branch_unreachable;
mod err_match_not_exhausted;
mod err_module_not_found;
mod err_scope_item_is_not_namespace;
mod err_scope_item_redeclared;
mod err_scope_item_undeclared;
mod err_scope_item_undeclared_enum_variant;
mod err_type_alias_does_not_point_to_structure;
mod err_type_alias_does_not_point_to_type;
mod err_use_expected_path;
mod ok_algorithm_factorial;
mod ok_algorithm_fibonacci;
mod ok_algorithm_modules;
mod ok_expression_array_nested;
mod ok_expression_block_mutating;
mod ok_expression_block_pyramid;
mod ok_expression_complex_mixed_items;
mod ok_expression_complex_mixed_types;
mod ok_expression_complex_operator;
mod ok_expression_conditional_elseless;
mod ok_expression_conditional_nested;
mod ok_expression_enum_different_types;
mod ok_expression_enum_enough_bitlength;
mod ok_expression_function_call;
mod ok_expression_function_procedure;
mod ok_expression_match_enum;
mod ok_expression_match_mutating;
mod ok_expression_match_nested;
mod ok_expression_structure_mutating;
mod ok_expression_structure_nested;
mod ok_expression_tuple_aliased;
mod ok_expression_tuple_nested;
mod ok_statement_impl_enum;
mod ok_statement_impl_struct;
mod ok_statement_loop_array;
mod ok_statement_loop_inclusive;
mod ok_statement_loop_nested;
mod ok_statement_loop_reverted;
mod ok_statement_loop_scope_popping;
mod ok_statement_loop_with_while;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use zinc_bytecode::Instruction;

use crate::BinaryAnalyzer;
use crate::Bytecode;
use crate::Error;
use crate::LibraryAnalyzer;
use crate::Parser;
use crate::Scope;

static PANIC_SYNTAX_ERROR: &str = "Syntax errors must be eliminated at this point";
static PANIC_ONLY_REFERENCE: &str = "The last shared reference is always unwrapped successfully";

pub(self) fn get_binary_result(input: &str) -> Result<(), Error> {
    BinaryAnalyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect(PANIC_SYNTAX_ERROR),
        HashMap::new(),
    )
}

pub(self) fn get_dependency(
    input: &str,
    bytecode: Rc<RefCell<Bytecode>>,
) -> Result<Rc<RefCell<Scope>>, Error> {
    LibraryAnalyzer::new(bytecode).compile(
        Parser::default()
            .parse(input.to_owned())
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
    BinaryAnalyzer::new(bytecode.clone()).compile(
        Parser::default()
            .parse(input.to_owned())
            .expect(PANIC_SYNTAX_ERROR),
        dependencies,
    )?;
    Ok(Rc::try_unwrap(bytecode)
        .expect(PANIC_ONLY_REFERENCE)
        .into_inner()
        .into_instructions())
}
