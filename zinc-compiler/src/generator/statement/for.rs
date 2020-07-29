//!
//! The generator `for` statement.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_bytecode::Instruction;

use crate::generator::expression::operand::block::Expression as BlockExpression;
use crate::generator::expression::operand::constant::boolean::Boolean as BooleanConstant;
use crate::generator::expression::operand::constant::integer::Integer as IntegerConstant;
use crate::generator::expression::Expression as GeneratorExpression;
use crate::generator::r#type::Type;
use crate::generator::state::State;
use crate::generator::IBytecodeWritable;
use crate::lexical::token::location::Location;

use num_bigint::BigInt;
use num_traits::One;

///
/// The Zinc VM loop statement.
///
#[derive(Debug, Clone)]
pub struct Statement {
    /// The statement location in the source code.
    pub location: Location,
    /// The initial value, which is assigned to the loop index variable before the first iteration.
    pub initial_value: BigInt,
    /// The number of loop iterations, usually the differece between the range bounds.
    pub iterations_count: usize,
    /// Whether the loop index is decreasing after each iteration.
    pub is_reversed: bool,
    /// The name of the loop index variable (like `i`).
    pub index_variable_name: String,
    /// If the loop index variable is signed, which happens if the range includes negative numbers.
    pub index_variable_is_signed: bool,
    /// The loop index variable bitlength, which is usually allocated to fit the bigger range bound.
    pub index_variable_bitlength: usize,
    /// The optional while condition, which can suppress the loop side effects if false.
    pub while_condition: Option<GeneratorExpression>,
    /// The loop body.
    pub body: BlockExpression,
}

impl Statement {
    ///
    /// A shortcut constructor.
    ///
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        location: Location,
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
            location,
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
}

impl IBytecodeWritable for Statement {
    fn write_all(self, bytecode: Rc<RefCell<State>>) {
        let index_type =
            Type::integer(self.index_variable_is_signed, self.index_variable_bitlength);
        let index_size = index_type.size();
        let index_address = bytecode
            .borrow_mut()
            .define_variable(Some(self.index_variable_name), index_size);
        IntegerConstant::new(
            self.initial_value.clone(),
            self.index_variable_is_signed,
            self.index_variable_bitlength,
        )
        .write_all(bytecode.clone());
        bytecode.borrow_mut().push_instruction(
            Instruction::Store(zinc_bytecode::Store::new(index_address, index_size)),
            Some(self.location),
        );

        let while_allowed_address = if self.while_condition.is_some() {
            let while_allowed = BooleanConstant::new(true);
            let while_allowed_address = bytecode
                .borrow_mut()
                .define_variable(None, Type::boolean().size());
            while_allowed.write_all(bytecode.clone());
            bytecode.borrow_mut().push_instruction(
                Instruction::Store(zinc_bytecode::Store::new(while_allowed_address, 1)),
                Some(self.location),
            );
            Some(while_allowed_address)
        } else {
            None
        };

        bytecode.borrow_mut().push_instruction(
            Instruction::LoopBegin(zinc_bytecode::LoopBegin::new(self.iterations_count)),
            Some(self.location),
        );

        if let (Some(while_condition), Some(while_allowed_address)) =
            (self.while_condition, while_allowed_address)
        {
            while_condition.write_all(bytecode.clone());
            bytecode
                .borrow_mut()
                .push_instruction(Instruction::Not(zinc_bytecode::Not), Some(self.location));
            bytecode
                .borrow_mut()
                .push_instruction(Instruction::If(zinc_bytecode::If), Some(self.location));
            BooleanConstant::new(false).write_all(bytecode.clone());
            bytecode.borrow_mut().push_instruction(
                Instruction::Store(zinc_bytecode::Store::new(
                    while_allowed_address,
                    Type::boolean().size(),
                )),
                Some(self.location),
            );
            bytecode.borrow_mut().push_instruction(
                Instruction::EndIf(zinc_bytecode::EndIf),
                Some(self.location),
            );

            bytecode.borrow_mut().push_instruction(
                Instruction::Load(zinc_bytecode::Load::new(
                    while_allowed_address,
                    Type::boolean().size(),
                )),
                Some(self.location),
            );
            bytecode
                .borrow_mut()
                .push_instruction(Instruction::If(zinc_bytecode::If), Some(self.location));
            self.body.write_all(bytecode.clone());
            bytecode.borrow_mut().push_instruction(
                Instruction::EndIf(zinc_bytecode::EndIf),
                Some(self.location),
            );
        } else {
            self.body.write_all(bytecode.clone());
        }

        if self.is_reversed {
            bytecode.borrow_mut().push_instruction(
                Instruction::Load(zinc_bytecode::Load::new(index_address, 1)),
                Some(self.location),
            );
            IntegerConstant::new_min(self.index_variable_is_signed, self.index_variable_bitlength)
                .write_all(bytecode.clone());
            bytecode
                .borrow_mut()
                .push_instruction(Instruction::Gt(zinc_bytecode::Gt), Some(self.location));
            bytecode
                .borrow_mut()
                .push_instruction(Instruction::If(zinc_bytecode::If), Some(self.location));
            bytecode.borrow_mut().push_instruction(
                Instruction::Load(zinc_bytecode::Load::new(index_address, 1)),
                Some(self.location),
            );
            IntegerConstant::new(
                BigInt::one(),
                self.index_variable_is_signed,
                self.index_variable_bitlength,
            )
            .write_all(bytecode.clone());
            bytecode
                .borrow_mut()
                .push_instruction(Instruction::Sub(zinc_bytecode::Sub), Some(self.location));
            bytecode.borrow_mut().push_instruction(
                Instruction::Store(zinc_bytecode::Store::new(index_address, 1)),
                Some(self.location),
            );
            bytecode.borrow_mut().push_instruction(
                Instruction::EndIf(zinc_bytecode::EndIf),
                Some(self.location),
            );
        } else {
            bytecode.borrow_mut().push_instruction(
                Instruction::Load(zinc_bytecode::Load::new(index_address, 1)),
                Some(self.location),
            );
            IntegerConstant::new_max(self.index_variable_is_signed, self.index_variable_bitlength)
                .write_all(bytecode.clone());
            bytecode
                .borrow_mut()
                .push_instruction(Instruction::Lt(zinc_bytecode::Lt), Some(self.location));
            bytecode
                .borrow_mut()
                .push_instruction(Instruction::If(zinc_bytecode::If), Some(self.location));
            bytecode.borrow_mut().push_instruction(
                Instruction::Load(zinc_bytecode::Load::new(index_address, 1)),
                Some(self.location),
            );
            IntegerConstant::new(
                BigInt::one(),
                self.index_variable_is_signed,
                self.index_variable_bitlength,
            )
            .write_all(bytecode.clone());
            bytecode
                .borrow_mut()
                .push_instruction(Instruction::Add(zinc_bytecode::Add), Some(self.location));
            bytecode.borrow_mut().push_instruction(
                Instruction::Store(zinc_bytecode::Store::new(index_address, 1)),
                Some(self.location),
            );
            bytecode.borrow_mut().push_instruction(
                Instruction::EndIf(zinc_bytecode::EndIf),
                Some(self.location),
            );
        };
        bytecode.borrow_mut().push_instruction(
            Instruction::LoopEnd(zinc_bytecode::LoopEnd),
            Some(self.location),
        );
    }
}
