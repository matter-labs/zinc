//!
//! The generator expression match operand.
//!

pub mod builder;

use crate::generator::expression::operand::constant::Constant;
use crate::generator::expression::Expression as GeneratorExpression;

#[derive(Debug, Clone)]
pub struct Expression {
    scrutinee: GeneratorExpression,
    branches: Vec<(Constant, GeneratorExpression)>,
    else_branch: GeneratorExpression,
}

impl Expression {
    pub fn new(
        scrutinee: GeneratorExpression,
        branches: Vec<(Constant, GeneratorExpression)>,
        else_branch: GeneratorExpression,
    ) -> Self {
        Self {
            scrutinee,
            branches,
            else_branch,
        }
    }

    fn _temp() {
        // let scrutinee_address = self
        //     .bytecode
        //     .borrow_mut()
        //     .allocate_data_stack_space(scrutinee_size);
        // self.bytecode.borrow_mut().push_instruction_store(
        //     scrutinee_address,
        //     scrutinee_size,
        //     None,
        //     scrutinee_location,
        // );

        // if index > 0 {
        //     self.bytecode
        //         .borrow_mut()
        //         .push_instruction(Instruction::Else(zinc_bytecode::Else), location);
        //     endifs += 1;
        // }
        //
        // self.bytecode.borrow_mut().push_instruction_load(
        //     scrutinee_address,
        //     scrutinee_size,
        //     None,
        //     scrutinee_location,
        // );
        // self.bytecode
        //     .borrow_mut()
        //     .push_instruction(constant.to_instruction(), location);
        // self.bytecode
        //     .borrow_mut()
        //     .push_instruction(Instruction::Eq(zinc_bytecode::Eq), location);
        // self.bytecode
        //     .borrow_mut()
        //     .push_instruction(Instruction::If(zinc_bytecode::If), location);

        // if index > 0 {
        //     self.bytecode
        //         .borrow_mut()
        //         .push_instruction(Instruction::Else(zinc_bytecode::Else), location);
        //     endifs += 1;
        // }
        //
        // self.bytecode.borrow_mut().push_instruction_load(
        //     scrutinee_address,
        //     scrutinee_size,
        //     None,
        //     scrutinee_location,
        // );
        // self.bytecode
        //     .borrow_mut()
        //     .push_instruction(constant.to_instruction(), location);
        // self.bytecode
        //     .borrow_mut()
        //     .push_instruction(Instruction::Eq(zinc_bytecode::Eq), location);
        // self.bytecode
        //     .borrow_mut()
        //     .push_instruction(Instruction::If(zinc_bytecode::If), location);

        // if index > 0 {
        //     self.bytecode
        //         .borrow_mut()
        //         .push_instruction(Instruction::Else(zinc_bytecode::Else), location);
        //     endifs += 1;
        // }

        // self.bytecode.borrow_mut().push_instruction_load(
        //     scrutinee_address,
        //     scrutinee_size,
        //     None,
        //     scrutinee_location,
        // );
        // match Scope::resolve_path(self.scope(), &path)?.variant {
        //     ScopeItem::Variable(variable) => {
        //         self.bytecode.borrow_mut().push_instruction_load(
        //             0, /* TODO */
        //             variable.r#type.size(),
        //             None,
        //             location,
        //         )
        //     }
        //     ScopeItem::Constant(constant) => self
        //         .bytecode
        //         .borrow_mut()
        //         .push_instruction(constant.to_instruction(), location),
        //     item => {
        //         return Err(Error::MatchBranchPatternPathExpectedEvaluable {
        //             location: path.location,
        //             found: item.to_string(),
        //         });
        //     }
        // }
        // self.bytecode
        //     .borrow_mut()
        //     .push_instruction(Instruction::Eq(zinc_bytecode::Eq), location);
        // self.bytecode
        //     .borrow_mut()
        //     .push_instruction(Instruction::If(zinc_bytecode::If), location);

        // if index > 0 {
        //     self.bytecode
        //         .borrow_mut()
        //         .push_instruction(Instruction::Else(zinc_bytecode::Else), location);
        // }

        // if index > 0 {
        //     self.bytecode
        //         .borrow_mut()
        //         .push_instruction(Instruction::EndIf(zinc_bytecode::EndIf), location);
        // }

        // if index > 0 {
        //     self.bytecode
        //         .borrow_mut()
        //         .push_instruction(Instruction::Else(zinc_bytecode::Else), location);
        // }

        // if index > 0 {
        //     self.bytecode
        //         .borrow_mut()
        //         .push_instruction(Instruction::EndIf(zinc_bytecode::EndIf), location);
        // }

        // for _ in 0..endifs {
        //     self.bytecode
        //         .borrow_mut()
        //         .push_instruction(Instruction::EndIf(zinc_bytecode::EndIf), location);
        // }
    }
}
