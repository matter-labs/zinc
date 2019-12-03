//!
//! A semantic analyzer test.
//!

mod error_conditional_branch_type_mismatch;
mod error_conditional_expected_boolean;
mod error_element_expected_place_expression;
mod error_element_let_casting;
mod error_function_argument_count_mismatch;
mod error_function_argument_type_mismatch;
mod error_function_calling_not_callable;
mod error_function_main_missing;
mod error_function_return_type_mismatch;
mod error_scope_mutating_immutable;
mod error_scope_redeclared_item;
mod ok_algorithm_factorial;
mod ok_algorithm_fibonacci;
mod ok_expression_block_mutating;
mod ok_expression_block_pyramid;
mod ok_expression_complex_operator;
mod ok_expression_conditional_elseless;
mod ok_expression_conditional_nested;
mod ok_expression_function_call;
mod ok_expression_function_procedure;
mod ok_statement_enum;
mod ok_statement_let;
mod ok_statement_loop_inclusive;
mod ok_statement_loop_nested;
mod ok_statement_loop_reverted;
mod ok_statement_loop_scope_popping;
mod ok_statement_mod;
mod ok_statement_struct;
mod ok_statement_type;
mod ok_statement_use;

#[allow(dead_code)]
static PANIC_SYNTAX_ERROR: &str = "Syntax errors must be eliminated at this point";

//mod error_enumeration_variant_not_exists;
//mod error_loop_while_expected_boolean;
//mod error_scope_addressing_primitive_variable;
//mod error_scope_array_access_as_structure;
//mod error_scope_array_access_as_tuple;
//mod error_scope_array_index_out_of_range;
//mod error_scope_structure_access_as_array;
//mod error_scope_structure_access_as_tuple;
//mod error_scope_structure_field_not_exists;
//mod error_scope_tuple_access_as_array;
//mod error_scope_tuple_access_as_structure;
//mod error_scope_tuple_field_not_exists;
//mod ok_expression_array_nested;
//mod ok_expression_complex_mixed;
//mod ok_expression_match_mutating;
//mod ok_expression_match_nested;
//mod ok_expression_structure_mutating;
//mod ok_expression_structure_nested;
//mod ok_expression_tuple_aliased;
//mod ok_expression_tuple_nested;
//mod ok_statement_loop_with_while;
