//!
//! The generator function statement.
//!

use crate::generator::expression::operand::block::Expression;

#[derive(Debug, Clone)]
pub struct Statement {
    pub input_size: usize,
    pub body: Expression,
    pub output_size: usize,
}

impl Statement {
    pub fn new(input_size: usize, body: Expression, output_size: usize) -> Self {
        Self {
            input_size,
            body,
            output_size,
        }
    }

    fn _temp() {
        // record the function address in the bytecode
        // self.bytecode
        //     .borrow_mut()
        //     .start_new_function(&statement.identifier.name, unique_id);

        // let address = self
        //     .bytecode
        //     .borrow_mut()
        //     .allocate_data_stack_space(r#type.size());

        // self.bytecode.borrow_mut().push_instruction(
        //     Instruction::Return(zinc_bytecode::Return::new(expected_type.size())),
        //     return_expression_location,
        // );
    }
}
