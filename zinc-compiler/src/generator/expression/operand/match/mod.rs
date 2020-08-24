//!
//! The generator expression match operand.
//!

pub mod builder;

use std::cell::RefCell;
use std::rc::Rc;

use zinc_build::Instruction;

use crate::generator::expression::operand::constant::Constant;
use crate::generator::expression::Expression as GeneratorExpression;
use crate::generator::r#type::Type;
use crate::generator::state::State;
use crate::generator::IBytecodeWritable;
use crate::lexical::token::location::Location;

///
/// The match expression which is translated to Zinc VM conditional series.
///
#[derive(Debug, Clone)]
pub struct Expression {
    /// The `match` expression location.
    location: Location,
    /// The scrutinee (matched) expression.
    scrutinee: GeneratorExpression,
    /// The scrutinee (matched) expression type.
    scrutinee_type: Type,
    /// The branches ordered array, where each branch consists of a pattern and result expression.
    branches: Vec<(Constant, GeneratorExpression)>,
    /// The binding branch, which is the last fallback branch.
    binding_branch: Option<(GeneratorExpression, String)>,
    /// The wildcard `_` branch, which is the last fallback branch. Ignored if `binding_branch` is set.
    wildcard_branch: Option<GeneratorExpression>,
}

impl Expression {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        location: Location,
        scrutinee: GeneratorExpression,
        scrutinee_type: Type,
        branches: Vec<(Constant, GeneratorExpression)>,
        binding_branch: Option<(GeneratorExpression, String)>,
        wildcard_branch: Option<GeneratorExpression>,
    ) -> Self {
        Self {
            location,
            scrutinee,
            scrutinee_type,
            branches,
            binding_branch,
            wildcard_branch,
        }
    }
}

impl IBytecodeWritable for Expression {
    fn write_all(self, bytecode: Rc<RefCell<State>>) {
        let branch_count = self.branches.len();
        let scrutinee_size = self.scrutinee_type.size();

        let (binding_branch, binding_name) = match self.binding_branch {
            Some((binding_branch, binding_name)) => (Some(binding_branch), Some(binding_name)),
            None => (None, None),
        };

        let scrutinee_address = bytecode
            .borrow_mut()
            .define_variable(binding_name, scrutinee_size);

        self.scrutinee.write_all(bytecode.clone());
        bytecode.borrow_mut().push_instruction(
            Instruction::Store(zinc_build::Store::new(scrutinee_address, scrutinee_size)),
            Some(self.location),
        );

        for (branch_pattern, branch_expression) in self.branches.into_iter() {
            bytecode.borrow_mut().push_instruction(
                Instruction::Load(zinc_build::Load::new(scrutinee_address, scrutinee_size)),
                Some(self.location),
            );
            branch_pattern.write_all(bytecode.clone());
            bytecode
                .borrow_mut()
                .push_instruction(Instruction::Eq(zinc_build::Eq), Some(self.location));
            bytecode
                .borrow_mut()
                .push_instruction(Instruction::If(zinc_build::If), Some(self.location));
            branch_expression.write_all(bytecode.clone());
            bytecode
                .borrow_mut()
                .push_instruction(Instruction::Else(zinc_build::Else), Some(self.location));
        }

        if let Some(binding_branch) = binding_branch {
            binding_branch.write_all(bytecode.clone());
        } else if let Some(wildcard_branch) = self.wildcard_branch {
            wildcard_branch.write_all(bytecode.clone());
        }

        bytecode
            .borrow_mut()
            .push_instruction(Instruction::EndIf(zinc_build::EndIf), Some(self.location));
        for _ in 0..branch_count - 1 {
            bytecode
                .borrow_mut()
                .push_instruction(Instruction::EndIf(zinc_build::EndIf), Some(self.location));
        }
    }
}
