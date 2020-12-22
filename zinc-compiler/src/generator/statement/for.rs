//!
//! The generator `for` statement.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Location;
use zinc_types::Instruction;

use crate::generator::expression::operand::block::Expression as BlockExpression;
use crate::generator::expression::operand::constant::boolean::Boolean as BooleanConstant;
use crate::generator::expression::operand::constant::integer::Integer as IntegerConstant;
use crate::generator::expression::Expression as GeneratorExpression;
use crate::generator::r#type::Type;
use crate::generator::zinc_vm::State as ZincVMState;
use crate::generator::IBytecodeWritable;

use num::BigInt;
use num::One;

///
/// The generator `for` statement.
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
    fn write_to_zinc_vm(self, state: Rc<RefCell<ZincVMState>>) {
        let index_type =
            Type::integer(self.index_variable_is_signed, self.index_variable_bitlength);
        let index_size = index_type.size();
        let index_address = state
            .borrow_mut()
            .define_variable(Some(self.index_variable_name), index_size);
        IntegerConstant::new(
            self.initial_value.clone(),
            self.index_variable_is_signed,
            self.index_variable_bitlength,
        )
        .write_to_zinc_vm(state.clone());
        state.borrow_mut().push_instruction(
            Instruction::Store(zinc_types::Store::new(index_address, index_size)),
            Some(self.location),
        );

        let while_allowed_address = if self.while_condition.is_some() {
            let while_allowed = BooleanConstant::new(true);
            let while_allowed_address = state
                .borrow_mut()
                .define_variable(None, Type::boolean().size());
            while_allowed.write_to_zinc_vm(state.clone());
            state.borrow_mut().push_instruction(
                Instruction::Store(zinc_types::Store::new(while_allowed_address, 1)),
                Some(self.location),
            );
            Some(while_allowed_address)
        } else {
            None
        };

        state.borrow_mut().push_instruction(
            Instruction::LoopBegin(zinc_types::LoopBegin::new(self.iterations_count)),
            Some(self.location),
        );

        if let (Some(while_condition), Some(while_allowed_address)) =
            (self.while_condition, while_allowed_address)
        {
            while_condition.write_to_zinc_vm(state.clone());
            state
                .borrow_mut()
                .push_instruction(Instruction::Not(zinc_types::Not), Some(self.location));
            state
                .borrow_mut()
                .push_instruction(Instruction::If(zinc_types::If), Some(self.location));
            BooleanConstant::new(false).write_to_zinc_vm(state.clone());
            state.borrow_mut().push_instruction(
                Instruction::Store(zinc_types::Store::new(
                    while_allowed_address,
                    Type::boolean().size(),
                )),
                Some(self.location),
            );
            state
                .borrow_mut()
                .push_instruction(Instruction::EndIf(zinc_types::EndIf), Some(self.location));

            state.borrow_mut().push_instruction(
                Instruction::Load(zinc_types::Load::new(
                    while_allowed_address,
                    Type::boolean().size(),
                )),
                Some(self.location),
            );
            state
                .borrow_mut()
                .push_instruction(Instruction::If(zinc_types::If), Some(self.location));
            self.body.write_to_zinc_vm(state.clone());
            state
                .borrow_mut()
                .push_instruction(Instruction::EndIf(zinc_types::EndIf), Some(self.location));
        } else {
            self.body.write_to_zinc_vm(state.clone());
        }

        if self.is_reversed {
            state.borrow_mut().push_instruction(
                Instruction::Load(zinc_types::Load::new(index_address, 1)),
                Some(self.location),
            );
            IntegerConstant::new_min(self.index_variable_is_signed, self.index_variable_bitlength)
                .write_to_zinc_vm(state.clone());
            state
                .borrow_mut()
                .push_instruction(Instruction::Gt(zinc_types::Gt), Some(self.location));
            state
                .borrow_mut()
                .push_instruction(Instruction::If(zinc_types::If), Some(self.location));
            state.borrow_mut().push_instruction(
                Instruction::Load(zinc_types::Load::new(index_address, 1)),
                Some(self.location),
            );
            IntegerConstant::new(
                BigInt::one(),
                self.index_variable_is_signed,
                self.index_variable_bitlength,
            )
            .write_to_zinc_vm(state.clone());
            state
                .borrow_mut()
                .push_instruction(Instruction::Sub(zinc_types::Sub), Some(self.location));
            state.borrow_mut().push_instruction(
                Instruction::Store(zinc_types::Store::new(index_address, 1)),
                Some(self.location),
            );
            state
                .borrow_mut()
                .push_instruction(Instruction::EndIf(zinc_types::EndIf), Some(self.location));
        } else {
            state.borrow_mut().push_instruction(
                Instruction::Load(zinc_types::Load::new(index_address, 1)),
                Some(self.location),
            );
            IntegerConstant::new_max(self.index_variable_is_signed, self.index_variable_bitlength)
                .write_to_zinc_vm(state.clone());
            state
                .borrow_mut()
                .push_instruction(Instruction::Lt(zinc_types::Lt), Some(self.location));
            state
                .borrow_mut()
                .push_instruction(Instruction::If(zinc_types::If), Some(self.location));
            state.borrow_mut().push_instruction(
                Instruction::Load(zinc_types::Load::new(index_address, 1)),
                Some(self.location),
            );
            IntegerConstant::new(
                BigInt::one(),
                self.index_variable_is_signed,
                self.index_variable_bitlength,
            )
            .write_to_zinc_vm(state.clone());
            state
                .borrow_mut()
                .push_instruction(Instruction::Add(zinc_types::Add), Some(self.location));
            state.borrow_mut().push_instruction(
                Instruction::Store(zinc_types::Store::new(index_address, 1)),
                Some(self.location),
            );
            state
                .borrow_mut()
                .push_instruction(Instruction::EndIf(zinc_types::EndIf), Some(self.location));
        };
        state.borrow_mut().push_instruction(
            Instruction::LoopEnd(zinc_types::LoopEnd),
            Some(self.location),
        );
    }
}
