//!
//! The generator for-loop statement.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_bytecode::Instruction;

use crate::bytecode::Bytecode;
use crate::generator::expression::operand::block::Expression as BlockExpression;
use crate::generator::expression::operand::constant::Constant;
use crate::generator::expression::Expression as GeneratorExpression;
use crate::generator::r#type::Type;

use num_bigint::BigInt;
use num_traits::One;

#[derive(Debug, Clone)]
pub struct Statement {
    pub initial_value: BigInt,
    pub iterations_count: usize,
    pub is_reversed: bool,
    pub index_variable_name: String,
    pub index_variable_is_signed: bool,
    pub index_variable_bitlength: usize,
    pub while_condition: Option<GeneratorExpression>,
    pub body: BlockExpression,
}

impl Statement {
    pub fn new(
        initial_value: BigInt,
        iterations_count: usize,
        is_reversed: bool,
        index_variable_name: String,
        index_variable_is_signed: bool,
        index_variable_bitlength: usize,
        while_condition: Option<GeneratorExpression>,
        body: BlockExpression,
    ) -> Self {
        Self {
            initial_value,
            iterations_count,
            is_reversed,
            index_variable_name,
            index_variable_is_signed,
            index_variable_bitlength,
            while_condition,
            body,
        }
    }

    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<Bytecode>>) {
        let index_type =
            Type::integer(self.index_variable_is_signed, self.index_variable_bitlength);
        let index_size = index_type.size();
        let index_address = bytecode
            .borrow_mut()
            .declare_variable(Some(self.index_variable_name), index_type);
        Constant::new_integer(
            self.initial_value.clone(),
            self.index_variable_is_signed,
            self.index_variable_bitlength,
        )
        .write_all_to_bytecode(bytecode.clone());
        bytecode.borrow_mut().push_instruction(
            Instruction::StoreSequence(zinc_bytecode::StoreSequence::new(
                index_address,
                index_size,
            )),
            crate::lexical::Location::default(),
        );

        let while_allowed_address = if self.while_condition.is_some() {
            let while_allowed = Constant::new_boolean(true);
            let while_allowed_address = bytecode
                .borrow_mut()
                .declare_variable(None, while_allowed.r#type());
            while_allowed.write_all_to_bytecode(bytecode.clone());
            bytecode.borrow_mut().push_instruction(
                Instruction::Store(zinc_bytecode::Store::new(while_allowed_address)),
                crate::lexical::Location::default(),
            );
            Some(while_allowed_address)
        } else {
            None
        };

        bytecode.borrow_mut().push_instruction(
            Instruction::LoopBegin(zinc_bytecode::LoopBegin::new(self.iterations_count)),
            crate::lexical::Location::default(),
        );

        if let (Some(while_condition), Some(while_allowed_address)) =
            (self.while_condition, while_allowed_address)
        {
            while_condition.write_all_to_bytecode(bytecode.clone());
            bytecode.borrow_mut().push_instruction(
                Instruction::Not(zinc_bytecode::Not),
                crate::lexical::Location::default(),
            );
            bytecode.borrow_mut().push_instruction(
                Instruction::If(zinc_bytecode::If),
                crate::lexical::Location::default(),
            );
            Constant::new_boolean(false).write_all_to_bytecode(bytecode.clone());
            bytecode.borrow_mut().push_instruction(
                Instruction::StoreSequence(zinc_bytecode::StoreSequence::new(
                    while_allowed_address,
                    Type::boolean().size(),
                )),
                crate::lexical::Location::default(),
            );
            bytecode.borrow_mut().push_instruction(
                Instruction::EndIf(zinc_bytecode::EndIf),
                crate::lexical::Location::default(),
            );

            bytecode.borrow_mut().push_instruction(
                Instruction::LoadSequence(zinc_bytecode::LoadSequence::new(
                    while_allowed_address,
                    Type::boolean().size(),
                )),
                crate::lexical::Location::default(),
            );
            bytecode.borrow_mut().push_instruction(
                Instruction::If(zinc_bytecode::If),
                crate::lexical::Location::default(),
            );
            self.body.write_all_to_bytecode(bytecode.clone());
            bytecode.borrow_mut().push_instruction(
                Instruction::EndIf(zinc_bytecode::EndIf),
                crate::lexical::Location::default(),
            );
        } else {
            self.body.write_all_to_bytecode(bytecode.clone());
        }

        if self.is_reversed {
            bytecode.borrow_mut().push_instruction(
                Instruction::Load(zinc_bytecode::Load::new(index_address)),
                crate::lexical::Location::default(),
            );
            Constant::new_min(self.index_variable_is_signed, self.index_variable_bitlength)
                .write_all_to_bytecode(bytecode.clone());
            bytecode.borrow_mut().push_instruction(
                Instruction::Gt(zinc_bytecode::Gt),
                crate::lexical::Location::default(),
            );
            bytecode.borrow_mut().push_instruction(
                Instruction::If(zinc_bytecode::If),
                crate::lexical::Location::default(),
            );
            bytecode.borrow_mut().push_instruction(
                Instruction::Load(zinc_bytecode::Load::new(index_address)),
                crate::lexical::Location::default(),
            );
            Constant::new_integer(
                BigInt::one(),
                self.index_variable_is_signed,
                self.index_variable_bitlength,
            )
            .write_all_to_bytecode(bytecode.clone());
            bytecode.borrow_mut().push_instruction(
                Instruction::Sub(zinc_bytecode::Sub),
                crate::lexical::Location::default(),
            );
            bytecode.borrow_mut().push_instruction(
                Instruction::Store(zinc_bytecode::Store::new(index_address)),
                crate::lexical::Location::default(),
            );
            bytecode.borrow_mut().push_instruction(
                Instruction::EndIf(zinc_bytecode::EndIf),
                crate::lexical::Location::default(),
            );
        } else {
            bytecode.borrow_mut().push_instruction(
                Instruction::Load(zinc_bytecode::Load::new(index_address)),
                crate::lexical::Location::default(),
            );
            Constant::new_max(self.index_variable_is_signed, self.index_variable_bitlength)
                .write_all_to_bytecode(bytecode.clone());
            bytecode.borrow_mut().push_instruction(
                Instruction::Lt(zinc_bytecode::Lt),
                crate::lexical::Location::default(),
            );
            bytecode.borrow_mut().push_instruction(
                Instruction::If(zinc_bytecode::If),
                crate::lexical::Location::default(),
            );
            bytecode.borrow_mut().push_instruction(
                Instruction::Load(zinc_bytecode::Load::new(index_address)),
                crate::lexical::Location::default(),
            );
            Constant::new_integer(
                BigInt::one(),
                self.index_variable_is_signed,
                self.index_variable_bitlength,
            )
            .write_all_to_bytecode(bytecode.clone());
            bytecode.borrow_mut().push_instruction(
                Instruction::Add(zinc_bytecode::Add),
                crate::lexical::Location::default(),
            );
            bytecode.borrow_mut().push_instruction(
                Instruction::Store(zinc_bytecode::Store::new(index_address)),
                crate::lexical::Location::default(),
            );
            bytecode.borrow_mut().push_instruction(
                Instruction::EndIf(zinc_bytecode::EndIf),
                crate::lexical::Location::default(),
            );
        };
        bytecode.borrow_mut().push_instruction(
            Instruction::LoopEnd(zinc_bytecode::LoopEnd),
            crate::lexical::Location::default(),
        );
    }
}
