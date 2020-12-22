//!
//! The virtual machine circuit.
//!

pub mod facade;
pub mod output;
pub mod synthesizer;

use colored::Colorize;
use num::bigint::ToBigInt;
use num::BigInt;

use franklin_crypto::bellman::ConstraintSystem;

use crate::core::contract::storage::leaf::LeafVariant;
use crate::core::contract::storage::setup::Storage as SetupStorage;
use crate::core::counter::NamespaceCounter;
use crate::core::execution_state::block::branch::Branch;
use crate::core::execution_state::block::r#loop::Loop;
use crate::core::execution_state::block::Block;
use crate::core::execution_state::cell::Cell;
use crate::core::execution_state::function_frame::Frame;
use crate::core::execution_state::ExecutionState;
use crate::core::location::Location;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::error::MalformedBytecode;
use crate::gadgets;
use crate::gadgets::scalar::Scalar;
use crate::instructions::call_library::INativeCallable;
use crate::instructions::IExecutable;
use crate::IEngine;

pub struct State<E, CS>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    counter: NamespaceCounter<E, CS>,
    execution_state: ExecutionState<E>,
    outputs: Vec<Scalar<E>>,

    pub(crate) location: Location,
}

impl<E, CS> State<E, CS>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    pub fn new(cs: CS) -> Self {
        Self {
            counter: NamespaceCounter::new(cs),
            execution_state: ExecutionState::new(),
            outputs: vec![],

            location: Location::new(),
        }
    }

    pub fn run<CB, F>(
        &mut self,
        circuit: zinc_types::Circuit,
        input_values: Option<&[BigInt]>,
        mut instruction_callback: CB,
        mut check_cs: F,
    ) -> Result<Vec<Option<BigInt>>, Error>
    where
        CB: FnMut(&CS),
        F: FnMut(&CS) -> Result<(), Error>,
    {
        self.counter.cs.enforce(
            || "ONE * ONE = ONE (do this to avoid `unconstrained` error)",
            |zero| zero + CS::one(),
            |zero| zero + CS::one(),
            |zero| zero + CS::one(),
        );
        let one = Scalar::new_constant_usize(1, zinc_types::ScalarType::Boolean);
        self.condition_push(one)?;

        let input_size = circuit.input.size();
        self.init_root_frame(circuit.input, input_values)?;

        if let Err(error) = zinc_types::Call::new(circuit.address, input_size)
            .execute(self)
            .and(check_cs(&self.counter.cs))
        {
            log::error!("{}\nat {}", error, self.location.to_string().blue());
            return Err(error);
        }

        let mut step = 0;
        while self.execution_state.instruction_counter < circuit.instructions.len() {
            let namespace = format!(
                "step={}, addr={}",
                step, self.execution_state.instruction_counter
            );
            self.counter.cs.push_namespace(|| namespace);
            let instruction =
                circuit.instructions[self.execution_state.instruction_counter].clone();

            log::trace!(
                "{}:{} > {}",
                step,
                self.execution_state.instruction_counter,
                instruction,
            );

            self.execution_state.instruction_counter += 1;
            if let Err(error) = instruction.execute(self).and(check_cs(&self.counter.cs)) {
                log::error!("{}\nat {}", error, self.location.to_string().blue());
                return Err(error);
            }

            log::trace!("{}", self.execution_state);
            instruction_callback(&self.counter.cs);
            self.counter.cs.pop_namespace();
            step += 1;
        }

        self.get_outputs()
    }

    pub fn test(&mut self, circuit: zinc_types::Circuit, address: usize) -> Result<(), Error> {
        self.counter.cs.enforce(
            || "ONE * ONE = ONE (do this to avoid `unconstrained` error)",
            |zero| zero + CS::one(),
            |zero| zero + CS::one(),
            |zero| zero + CS::one(),
        );
        let one = Scalar::new_constant_usize(1, zinc_types::ScalarType::Boolean);
        self.condition_push(one)?;

        self.init_root_frame(zinc_types::Type::empty_structure(), Some(&[]))?;

        if let Err(error) = zinc_types::Call::new(address, 0).execute(self) {
            log::error!("{}\nat {}", error, self.location.to_string().blue());
            return Err(error);
        }

        let mut step = 0;
        while self.execution_state.instruction_counter < circuit.instructions.len() {
            let namespace = format!(
                "step={}, addr={}",
                step, self.execution_state.instruction_counter
            );
            self.counter.cs.push_namespace(|| namespace);
            let instruction =
                circuit.instructions[self.execution_state.instruction_counter].clone();

            log::trace!(
                "{}:{} > {}",
                step,
                self.execution_state.instruction_counter,
                instruction,
            );

            self.execution_state.instruction_counter += 1;
            if let Err(error) = instruction.execute(self) {
                log::error!("{}\nat {}", error, self.location.to_string().blue());
                return Err(error);
            }

            log::trace!("{}", self.execution_state);
            self.counter.cs.pop_namespace();
            step += 1;
        }

        Ok(())
    }

    fn init_root_frame(
        &mut self,
        input_type: zinc_types::Type,
        inputs: Option<&[BigInt]>,
    ) -> Result<(), Error> {
        self.execution_state
            .frames_stack
            .push(Frame::new(0, std::usize::MAX));
        let types = input_type.into_flat_scalar_types();

        // Convert Option<&[BigInt]> to iterator of Option<&BigInt> and zip with types.
        let value_type_pairs: Vec<_> = match inputs {
            Some(values) => values.iter().map(Option::Some).zip(types).collect(),
            None => std::iter::repeat(None).zip(types).collect(),
        };

        for (value, dtype) in value_type_pairs {
            let variable = gadgets::witness::allocate(self.counter.next(), value, dtype)?;
            self.push(Cell::Value(variable))?;
        }

        Ok(())
    }

    fn get_outputs(&mut self) -> Result<Vec<Option<BigInt>>, Error> {
        let outputs_fr: Vec<_> = self.outputs.iter().map(|f| (*f).clone()).collect();

        let mut outputs_bigint = Vec::with_capacity(outputs_fr.len());
        for output in outputs_fr.into_iter() {
            let output = gadgets::output::output(self.counter.next(), output)?;
            outputs_bigint.push(output.to_bigint());
        }

        Ok(outputs_bigint)
    }

    pub fn condition_push(&mut self, element: Scalar<E>) -> Result<(), Error> {
        self.execution_state.conditions_stack.push(element);
        Ok(())
    }

    pub fn condition_pop(&mut self) -> Result<Scalar<E>, Error> {
        self.execution_state
            .conditions_stack
            .pop()
            .ok_or_else(|| MalformedBytecode::StackUnderflow.into())
    }

    fn top_frame(&mut self) -> Result<&mut Frame<E>, Error> {
        self.execution_state
            .frames_stack
            .last_mut()
            .ok_or_else(|| MalformedBytecode::StackUnderflow.into())
    }
}

impl<E, CS> IVirtualMachine for State<E, CS>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    type E = E;
    type CS = CS;
    type S = SetupStorage<E>;

    fn push(&mut self, cell: Cell<E>) -> Result<(), Error> {
        self.execution_state.evaluation_stack.push(cell)
    }

    fn pop(&mut self) -> Result<Cell<E>, Error> {
        self.execution_state.evaluation_stack.pop()
    }

    fn load(&mut self, address: usize) -> Result<Cell<E>, Error> {
        let frame_start = self.top_frame()?.stack_frame_start;
        self.execution_state.data_stack.get(frame_start + address)
    }

    fn store(&mut self, address: usize, cell: Cell<E>) -> Result<(), Error> {
        let frame = self.top_frame()?;
        frame.stack_frame_end =
            std::cmp::max(frame.stack_frame_end, frame.stack_frame_start + address + 1);

        let frame_start = frame.stack_frame_start;

        self.execution_state
            .data_stack
            .set(frame_start + address, cell)
    }

    fn storage_init(
        &mut self,
        _project: zinc_project::ManifestProject,
        _values: Vec<Scalar<Self::E>>,
        _field_types: Vec<zinc_types::ContractFieldType>,
    ) -> Result<Scalar<Self::E>, Error> {
        Err(Error::OnlyForContracts)
    }

    fn storage_fetch(
        &mut self,
        _eth_address: Scalar<Self::E>,
        _field_types: Vec<zinc_types::ContractFieldType>,
    ) -> Result<(), Error> {
        Err(Error::OnlyForContracts)
    }

    fn storage_load(
        &mut self,
        _eth_address: Scalar<Self::E>,
        _address: Scalar<Self::E>,
        _size: usize,
    ) -> Result<Vec<Scalar<Self::E>>, Error> {
        Err(Error::OnlyForContracts)
    }

    fn storage_store(
        &mut self,
        _eth_address: Scalar<Self::E>,
        _address: Scalar<Self::E>,
        _value: LeafVariant<Self::E>,
    ) -> Result<(), Error> {
        Err(Error::OnlyForContracts)
    }

    fn storages_count(&self) -> usize {
        0
    }

    fn loop_begin(&mut self, iterations: usize) -> Result<(), Error> {
        let frame = self
            .execution_state
            .frames_stack
            .last_mut()
            .ok_or_else(|| Error::InternalError("Root frame is missing".into()))?;

        frame.blocks.push(Block::Loop(Loop {
            first_instruction_index: self.execution_state.instruction_counter,
            iterations_left: iterations - 1,
        }));

        Ok(())
    }

    fn loop_end(&mut self) -> Result<(), Error> {
        let frame = self
            .execution_state
            .frames_stack
            .last_mut()
            .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS);

        match frame.blocks.pop() {
            Some(Block::Loop(mut loop_block)) => {
                if loop_block.iterations_left != 0 {
                    loop_block.iterations_left -= 1;
                    self.execution_state.instruction_counter = loop_block.first_instruction_index;
                    frame.blocks.push(Block::Loop(loop_block));
                }
                Ok(())
            }
            _ => Err(MalformedBytecode::UnexpectedLoopEnd.into()),
        }
    }

    fn call(&mut self, address: usize, inputs_count: usize) -> Result<(), Error> {
        let offset = self.top_frame()?.stack_frame_end;
        self.execution_state
            .frames_stack
            .push(Frame::new(offset, self.execution_state.instruction_counter));

        for i in 0..inputs_count {
            let arg = self.pop()?;
            self.store(inputs_count - i - 1, arg)?;
        }

        self.execution_state.instruction_counter = address;
        Ok(())
    }

    fn r#return(&mut self, outputs_count: usize) -> Result<(), Error> {
        let mut outputs = Vec::with_capacity(outputs_count);
        for _ in 0..outputs_count {
            let output = self.pop()?;
            outputs.push(output);
        }

        let frame = self
            .execution_state
            .frames_stack
            .pop()
            .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS);

        if self.execution_state.frames_stack.len() == 1 {
            for cell in outputs.into_iter().rev() {
                self.outputs.push(cell.try_into_value()?);
            }

            self.execution_state.instruction_counter = std::usize::MAX;
        } else {
            for cell in outputs.into_iter().rev() {
                self.push(cell)?;
            }

            self.execution_state
                .data_stack
                .drop_from(frame.stack_frame_start);

            self.execution_state.instruction_counter = frame.return_address;
        }

        Ok(())
    }

    fn branch_then(&mut self) -> Result<(), Error> {
        let condition = self.pop()?.try_into_value()?;

        let prev = self.condition_top()?;

        let cs = self.constraint_system();
        let next = gadgets::logical::and::and(cs.namespace(|| "branch"), &condition, &prev)?;
        self.execution_state.conditions_stack.push(next);

        let branch = Branch {
            condition,
            is_else: false,
        };

        self.top_frame()?.blocks.push(Block::Branch(branch));

        self.execution_state.evaluation_stack.fork();
        self.execution_state.data_stack.fork();

        Ok(())
    }

    fn branch_else(&mut self) -> Result<(), Error> {
        let frame = self
            .execution_state
            .frames_stack
            .last_mut()
            .ok_or_else(|| Error::InternalError("Root frame is missing".into()))?;

        let mut branch = match frame.blocks.pop() {
            Some(Block::Branch(branch)) => Ok(branch),
            Some(_) | None => Err(Error::MalformedBytecode(MalformedBytecode::UnexpectedElse)),
        }?;

        if branch.is_else {
            return Err(MalformedBytecode::UnexpectedElse.into());
        } else {
            branch.is_else = true;
        }

        let condition = branch.condition.clone();

        frame.blocks.push(Block::Branch(branch));

        self.condition_pop()?;
        let prev = self.condition_top()?;
        let cs = self.constraint_system();
        let not_cond = gadgets::logical::not::not(cs.namespace(|| "not"), &condition)?;
        let next = gadgets::logical::and::and(cs.namespace(|| "and"), &prev, &not_cond)?;
        self.condition_push(next)?;

        self.execution_state.data_stack.switch_branch()?;
        self.execution_state.evaluation_stack.fork();

        Ok(())
    }

    fn branch_end(&mut self) -> Result<(), Error> {
        self.condition_pop()?;

        let frame = self
            .execution_state
            .frames_stack
            .last_mut()
            .ok_or_else(|| Error::InternalError("Root frame is missing".into()))?;

        let branch = match frame.blocks.pop() {
            Some(Block::Branch(branch)) => Ok(branch),
            Some(_) | None => Err(MalformedBytecode::UnexpectedEndIf),
        }?;

        if branch.is_else {
            self.execution_state
                .evaluation_stack
                .merge(self.counter.next(), &branch.condition)?;
        } else {
            self.execution_state.evaluation_stack.revert()?;
        }

        self.execution_state
            .data_stack
            .merge(self.counter.next(), branch.condition)?;

        Ok(())
    }

    fn call_native<F: INativeCallable<E, SetupStorage<E>>>(
        &mut self,
        function: F,
    ) -> Result<(), Error> {
        let state = &mut self.execution_state;
        let cs = &mut self.counter.cs;

        function.call(cs.namespace(|| "native function"), state, None)
    }

    fn condition_top(&mut self) -> Result<Scalar<E>, Error> {
        self.execution_state
            .conditions_stack
            .last()
            .map(|e| (*e).clone())
            .ok_or_else(|| MalformedBytecode::StackUnderflow.into())
    }

    fn constraint_system(&mut self) -> &mut CS {
        &mut self.counter.cs
    }

    fn get_location(&mut self) -> Location {
        self.location.clone()
    }

    fn set_location(&mut self, location: Location) {
        self.location = location;
    }
}
