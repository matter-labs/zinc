//!
//! The generator expression match operand.
//!

pub mod builder;

use std::cell::RefCell;
use std::rc::Rc;

use zinc_bytecode::Instruction;

use crate::bytecode::Bytecode;
use crate::generator::expression::operand::constant::Constant;
use crate::generator::expression::Expression as GeneratorExpression;
use crate::generator::r#type::Type;

#[derive(Debug, Clone)]
pub struct Expression {
    scrutinee: GeneratorExpression,
    scrutinee_type: Type,
    branches: Vec<(Constant, GeneratorExpression)>,
    binding_branch: Option<(GeneratorExpression, String)>,
    wildcard_branch: Option<GeneratorExpression>,
}

impl Expression {
    pub fn new(
        scrutinee: GeneratorExpression,
        scrutinee_type: Type,
        branches: Vec<(Constant, GeneratorExpression)>,
        binding_branch: Option<(GeneratorExpression, String)>,
        wildcard_branch: Option<GeneratorExpression>,
    ) -> Self {
        Self {
            scrutinee,
            scrutinee_type,
            branches,
            binding_branch,
            wildcard_branch,
        }
    }

    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<Bytecode>>) {
        let branch_count = self.branches.len();
        let scrutinee_size = self.scrutinee_type.size();

        let (binding_branch, binding_name) = match self.binding_branch {
            Some((binding_branch, binding_name)) => (Some(binding_branch), Some(binding_name)),
            None => (None, None),
        };

        let scrutinee_address = bytecode
            .borrow_mut()
            .declare_variable(binding_name, self.scrutinee_type.clone());

        self.scrutinee.write_all_to_bytecode(bytecode.clone());
        bytecode.borrow_mut().push_instruction(
            Instruction::StoreSequence(zinc_bytecode::StoreSequence::new(
                scrutinee_address,
                scrutinee_size,
            )),
            crate::lexical::Location::default(),
        );
        for (branch_pattern, branch_expression) in self.branches.into_iter() {
            bytecode.borrow_mut().push_instruction(
                Instruction::LoadSequence(zinc_bytecode::LoadSequence::new(
                    scrutinee_address,
                    scrutinee_size,
                )),
                crate::lexical::Location::default(),
            );
            branch_pattern.write_all_to_bytecode(bytecode.clone());
            bytecode.borrow_mut().push_instruction(
                Instruction::Eq(zinc_bytecode::Eq),
                crate::lexical::Location::default(),
            );
            bytecode.borrow_mut().push_instruction(
                Instruction::If(zinc_bytecode::If),
                crate::lexical::Location::default(),
            );
            branch_expression.write_all_to_bytecode(bytecode.clone());
            bytecode.borrow_mut().push_instruction(
                Instruction::Else(zinc_bytecode::Else),
                crate::lexical::Location::default(),
            );
        }
        if let Some(binding_branch) = binding_branch {
            binding_branch.write_all_to_bytecode(bytecode.clone());
        } else if let Some(wildcard_branch) = self.wildcard_branch {
            wildcard_branch.write_all_to_bytecode(bytecode.clone());
        }
        bytecode.borrow_mut().push_instruction(
            Instruction::EndIf(zinc_bytecode::EndIf),
            crate::lexical::Location::default(),
        );
        for _ in 0..branch_count - 1 {
            bytecode.borrow_mut().push_instruction(
                Instruction::EndIf(zinc_bytecode::EndIf),
                crate::lexical::Location::default(),
            );
        }
    }
}
