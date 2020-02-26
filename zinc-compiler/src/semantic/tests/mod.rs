//!
//! A semantic analyzer test.
//!

#![allow(dead_code)]

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
mod err_element_constant_integer_overflow_addition_signed_negative;
mod err_element_constant_integer_overflow_addition_signed_positive;
mod err_element_constant_integer_overflow_addition_unsigned_positive;
mod err_element_constant_integer_overflow_casting_signed_positive;
mod err_element_constant_integer_overflow_casting_unsigned_negative;
mod err_element_constant_integer_overflow_division_signed_positive;
mod err_element_constant_integer_overflow_multiplication_signed_negative;
mod err_element_constant_integer_overflow_multiplication_signed_positive;
mod err_element_constant_integer_overflow_multiplication_unsigned_positive;
mod err_element_constant_integer_overflow_negation_signed_positive;
mod err_element_constant_integer_overflow_negation_unsigned_negative;
mod err_element_constant_integer_overflow_subtraction_signed_negative;
mod err_element_constant_integer_overflow_subtraction_signed_positive;
mod err_element_constant_integer_overflow_subtraction_unsigned_negative;
mod err_element_constant_integer_types_mismatch_addition;
mod err_element_constant_integer_types_mismatch_division;
mod err_element_constant_integer_types_mismatch_greater;
mod err_element_constant_integer_types_mismatch_greater_equals;
mod err_element_constant_integer_types_mismatch_lesser;
mod err_element_constant_integer_types_mismatch_lesser_equals;
mod err_element_constant_integer_types_mismatch_multiplication;
mod err_element_constant_integer_types_mismatch_remainder;
mod err_element_constant_integer_types_mismatch_subtraction;
mod err_element_constant_integer_zero_division;
mod err_element_constant_integer_zero_remainder;
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
mod err_element_constant_range_1st_expected_integer;
mod err_element_constant_range_2nd_expected_integer;
mod err_element_constant_range_inclusive_1st_expected_integer;
mod err_element_constant_range_inclusive_2nd_expected_integer;
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
mod err_element_place_index_slice_end_lesser_than_start;
mod err_element_place_index_slice_end_out_of_range;
mod err_element_place_index_slice_start_out_of_range;
mod err_element_range_1st_expected_constant;
mod err_element_range_2nd_expected_constant;
mod err_element_range_inclusive_1st_expected_constant;
mod err_element_range_inclusive_2nd_expected_constant;
mod err_element_remainder_1st_expected_evaluable;
mod err_element_remainder_2nd_expected_evaluable;
mod err_element_subtraction_1st_expected_evaluable;
mod err_element_subtraction_2nd_expected_evaluable;
mod err_element_value_addition_1st_expected_integer;
mod err_element_value_addition_2nd_expected_integer;
mod err_element_value_and_1st_expected_boolean;
mod err_element_value_and_2nd_expected_boolean;
mod err_element_value_array_pushing_invalid_type;
mod err_element_value_array_slice_end_lesser_than_start;
mod err_element_value_array_slice_end_out_of_range;
mod err_element_value_array_slice_start_out_of_range;
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
mod err_element_value_index_2nd_expected_integer_or_range;
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
mod err_function_argument_count;
mod err_function_argument_type;
mod err_function_calling_not_callable;
mod err_function_instruction_specifier_missing;
mod err_function_instruction_unknown;
mod err_function_main_missing;
mod err_function_return_type_mismatch;
mod err_impl_expected_structure_or_enumeration;
mod err_inference_constant;
mod err_inference_constant_loop_bounds;
mod err_inference_constant_pattern_match;
mod err_literal_structure_field_does_not_exist;
mod err_literal_structure_field_invalid_type;
mod err_loop_bounds_expected_constant_range_expression;
mod err_loop_while_expected_boolean_condition;
mod err_match_branch_expression_invalid_type;
mod err_match_branch_pattern_invalid_type;
mod err_match_branch_pattern_path_expected_evaluable;
mod err_match_branch_unreachable;
mod err_match_not_exhausted;
mod err_module_not_found;
mod err_mutating_immutable_memory;
mod err_mutating_with_different_type;
mod err_scope_item_is_not_namespace;
mod err_scope_item_redeclared;
mod err_scope_item_undeclared;
mod err_scope_item_undeclared_enum_variant;
mod err_type_alias_does_not_point_to_structure;
mod err_type_alias_does_not_point_to_type;
mod err_use_expected_path;

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
static PANIC_THE_ONLY_REFERENCE: &str =
    "The last shared reference is always unwrapped successfully";

pub(self) fn get_binary_result(input: &str) -> Result<(), Error> {
    BinaryAnalyzer::default().compile(
        Parser::default()
            .parse(input, None)
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
    BinaryAnalyzer::new(bytecode.clone()).compile(
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
