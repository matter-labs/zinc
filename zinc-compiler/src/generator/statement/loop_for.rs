//!
//! The generator for-loop statement.
//!

use crate::generator::expression::operand::block::Expression as BlockExpression;
use crate::generator::expression::Expression as GeneratorExpression;

use num_bigint::BigInt;

#[derive(Debug, Clone)]
pub struct Statement {
    pub start: BigInt,
    pub end: BigInt,
    pub is_inclusive: bool,
    pub while_condition: Option<GeneratorExpression>,
    pub body: BlockExpression,
}

impl Statement {
    pub fn new(
        start: BigInt,
        end: BigInt,
        is_inclusive: bool,
        while_condition: Option<GeneratorExpression>,
        body: BlockExpression,
    ) -> Self {
        Self {
            start,
            end,
            is_inclusive,
            while_condition,
            body,
        }
    }

    fn _temp() {
        // create the index value and get its address
        // let index = IntegerConstant::new(range_start.clone(), is_signed, bitlength);
        // let index_type = index.r#type();
        // let index_size = index_type.size();
        // let index_address = self
        //     .bytecode
        //     .borrow_mut()
        //     .allocate_data_stack_space(index_size);
        // self.bytecode
        //     .borrow_mut()
        //     .push_instruction(index.to_instruction(), bounds_expression_location);
        // self.bytecode.borrow_mut().push_instruction_store(
        //     index_address,
        //     index_size,
        //     None,
        //     bounds_expression_location,
        // );

        // create the while allowed condition
        // let while_allowed_address = match statement.while_condition {
        //     Some(ref condition) => {
        //         let while_allowed = Constant::Boolean(true);
        //         let while_allowed_address = self
        //             .bytecode
        //             .borrow_mut()
        //             .allocate_data_stack_space(while_allowed.r#type().size());
        //         self.bytecode
        //             .borrow_mut()
        //             .push_instruction(while_allowed.to_instruction(), condition.location);
        //         self.bytecode.borrow_mut().push_instruction(
        //             Instruction::Store(zinc_bytecode::Store::new(while_allowed_address)),
        //             condition.location,
        //         );
        //         Some(while_allowed_address)
        //     }
        //     None => None,
        // };

        // calculate the iterations number and if the loop is reverse
        // let iterations_count = cmp::max(&range_start, &range_end)
        //     - cmp::min(&range_start, &range_end)
        //     + if is_inclusive {
        //     BigInt::one()
        // } else {
        //     BigInt::zero()
        // };
        // let is_reverse = range_start > range_end;
        //
        // let iterations_count = iterations_count.to_usize().ok_or_else(|| {
        //     Error::Element(
        //         location,
        //         ElementError::Constant(ConstantError::Integer(
        //             IntegerConstantError::IntegerTooLarge {
        //                 value: iterations_count.to_owned(),
        //                 bitlength: crate::BITLENGTH_INDEX,
        //             },
        //         )),
        //     )
        // })?;
        // self.bytecode.borrow_mut().push_instruction(
        //     Instruction::LoopBegin(zinc_bytecode::LoopBegin::new(iterations_count)),
        //     bounds_expression_location,
        // );

        // self.bytecode
        //     .borrow_mut()
        //     .push_instruction(Instruction::Not(zinc_bytecode::Not), location);
        // self.bytecode
        //     .borrow_mut()
        //     .push_instruction(Instruction::If(zinc_bytecode::If), location);
        // self.bytecode
        //     .borrow_mut()
        //     .push_instruction(Constant::Boolean(false).to_instruction(), location);
        // self.bytecode.borrow_mut().push_instruction_store(
        //     while_allowed_address,
        //     Type::boolean().size(),
        //     None,
        //     location,
        // );
        // self.bytecode
        //     .borrow_mut()
        //     .push_instruction(Instruction::EndIf(zinc_bytecode::EndIf), location);
        //
        // self.bytecode.borrow_mut().push_instruction_load(
        //     while_allowed_address,
        //     Type::boolean().size(),
        //     None,
        //     location,
        // );
        // self.bytecode
        //     .borrow_mut()
        //     .push_instruction(Instruction::If(zinc_bytecode::If), location);

        // self.bytecode
        //     .borrow_mut()
        //     .push_instruction(Instruction::EndIf(zinc_bytecode::EndIf), location);

        // increment or decrement the loop counter
        // if is_reverse {
        //     self.bytecode.borrow_mut().push_instruction(
        //         Instruction::Load(zinc_bytecode::Load::new(index_address)),
        //         location,
        //     );
        //     self.bytecode.borrow_mut().push_instruction(
        //         IntegerConstant::new_min(is_signed, bitlength).to_instruction(),
        //         location,
        //     );
        //     self.bytecode
        //         .borrow_mut()
        //         .push_instruction(Instruction::Gt(zinc_bytecode::Gt), location);
        //     self.bytecode
        //         .borrow_mut()
        //         .push_instruction(Instruction::If(zinc_bytecode::If), location);
        //     self.bytecode.borrow_mut().push_instruction(
        //         Instruction::Load(zinc_bytecode::Load::new(index_address)),
        //         location,
        //     );
        //     self.bytecode.borrow_mut().push_instruction(
        //         IntegerConstant::new_one(is_signed, bitlength).to_instruction(),
        //         location,
        //     );
        //     self.bytecode
        //         .borrow_mut()
        //         .push_instruction(Instruction::Sub(zinc_bytecode::Sub), location);
        //     self.bytecode.borrow_mut().push_instruction(
        //         Instruction::Store(zinc_bytecode::Store::new(index_address)),
        //         location,
        //     );
        //     self.bytecode
        //         .borrow_mut()
        //         .push_instruction(Instruction::EndIf(zinc_bytecode::EndIf), location);
        // } else {
        //     self.bytecode.borrow_mut().push_instruction(
        //         Instruction::Load(zinc_bytecode::Load::new(index_address)),
        //         location,
        //     );
        //     self.bytecode.borrow_mut().push_instruction(
        //         IntegerConstant::new_max(is_signed, bitlength).to_instruction(),
        //         location,
        //     );
        //     self.bytecode
        //         .borrow_mut()
        //         .push_instruction(Instruction::Lt(zinc_bytecode::Lt), location);
        //     self.bytecode
        //         .borrow_mut()
        //         .push_instruction(Instruction::If(zinc_bytecode::If), location);
        //     self.bytecode.borrow_mut().push_instruction(
        //         Instruction::Load(zinc_bytecode::Load::new(index_address)),
        //         location,
        //     );
        //     self.bytecode.borrow_mut().push_instruction(
        //         IntegerConstant::new_one(is_signed, bitlength).to_instruction(),
        //         location,
        //     );
        //     self.bytecode
        //         .borrow_mut()
        //         .push_instruction(Instruction::Add(zinc_bytecode::Add), location);
        //     self.bytecode.borrow_mut().push_instruction(
        //         Instruction::Store(zinc_bytecode::Store::new(index_address)),
        //         location,
        //     );
        //     self.bytecode
        //         .borrow_mut()
        //         .push_instruction(Instruction::EndIf(zinc_bytecode::EndIf), location);
        // };
        // self.bytecode
        //     .borrow_mut()
        //     .push_instruction(Instruction::LoopEnd(zinc_bytecode::LoopEnd), location);
    }
}
