//!
//! The virtual machine contract.
//!

pub mod facade;
pub mod storage;
pub mod synthesizer;

use colored::Colorize;
use num_bigint::BigInt;
use num_bigint::ToBigInt;

use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::DataType;
use zinc_bytecode::Program as BytecodeProgram;
use zinc_bytecode::ScalarType;

use crate::core::counter::NamespaceCounter;
use crate::core::execution_state::block::branch::Branch;
use crate::core::execution_state::block::r#loop::Loop;
use crate::core::execution_state::block::Block;
use crate::core::execution_state::cell::Cell;
use crate::core::execution_state::function_frame::Frame;
use crate::core::execution_state::ExecutionState;
use crate::core::location::Location;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::MalformedBytecode;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::gadgets::contract::merkle_tree::hasher::IHasher as IMerkleTreeHasher;
use crate::gadgets::contract::merkle_tree::IMerkleTree;
use crate::gadgets::contract::storage::StorageGadget;
use crate::gadgets::scalar::Scalar;
use crate::instructions::call_std::INativeCallable;
use crate::instructions::IExecutable;
use crate::IEngine;

pub struct Contract<E, CS, S, H>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
    S: IMerkleTree<E>,
    H: IMerkleTreeHasher<E>,
{
    counter: NamespaceCounter<E, CS>,
    state: ExecutionState<E>,
    outputs: Vec<Scalar<E>>,
    storage: StorageGadget<E, S, H>,

    pub(crate) debugging: bool,
    pub(crate) location: Location,
}

impl<E, CS, S, H> Contract<E, CS, S, H>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
    S: IMerkleTree<E>,
    H: IMerkleTreeHasher<E>,
{
    pub fn new(cs: CS, storage: StorageGadget<E, S, H>, debugging: bool) -> Self {
        Self {
            counter: NamespaceCounter::new(cs),
            state: ExecutionState::new(),
            outputs: vec![],
            storage,

            debugging,
            location: Location::new(),
        }
    }

    pub fn run<CB, F>(
        &mut self,
        bytecode: &BytecodeProgram,
        inputs: Option<&[BigInt]>,
        mut instruction_callback: CB,
        mut check_cs: F,
    ) -> Result<Vec<Option<BigInt>>, RuntimeError>
    where
        CB: FnMut(&CS) -> (),
        F: FnMut(&CS) -> Result<(), RuntimeError>,
    {
        self.counter.cs.enforce(
            || "ONE * ONE = ONE (do this to avoid `unconstrained` error)",
            |zero| zero + CS::one(),
            |zero| zero + CS::one(),
            |zero| zero + CS::one(),
        );
        let one = Scalar::new_constant_bigint(&1.into(), ScalarType::Boolean)?;
        self.condition_push(one)?;

        self.init_root_frame(&bytecode.input(), inputs)?;
        self.init_storage()?;

        let mut step = 0;
        while self.state.instruction_counter < bytecode.instructions().len() {
            let namespace = format!("step={}, addr={}", step, self.state.instruction_counter);
            self.counter.cs.push_namespace(|| namespace);
            let instruction = &bytecode.instructions()[self.state.instruction_counter];

            let log_message = format!(
                "{}:{} > {}",
                step, self.state.instruction_counter, instruction,
            );
            if instruction.is_debug() {
                log::debug!("{}", log_message);
            } else {
                log::info!("{}", log_message);
            }

            self.state.instruction_counter += 1;
            let result = instruction.execute(self);
            if let Err(err) = result.and(check_cs(&self.counter.cs)) {
                log::error!("{}\nat {}", err, self.location.to_string().blue());
                return Err(err);
            }

            log::trace!("{}", self.state);
            instruction_callback(&self.counter.cs);
            self.counter.cs.pop_namespace();
            step += 1;
        }

        self.get_outputs()
    }

    fn init_storage(&mut self) -> Result<(), RuntimeError> {
        // Temporary fix to avoid "unconstrained" error
        let root_hash = self.storage.root_hash()?;
        let cs = self.constraint_system();

        gadgets::arithmetic::add::add(
            cs.namespace(|| "root_hash constraint"),
            &root_hash,
            &Scalar::new_constant_int(0, ScalarType::Field),
        )?;

        Ok(())
    }

    fn init_root_frame(
        &mut self,
        input_type: &DataType,
        inputs: Option<&[BigInt]>,
    ) -> Result<(), RuntimeError> {
        self.state.frames_stack.push(Frame::new(0, std::usize::MAX));

        let types = input_type.to_scalar_types();

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

    fn get_outputs(&mut self) -> Result<Vec<Option<BigInt>>, RuntimeError> {
        let outputs_fr: Vec<_> = self.outputs.iter().map(|f| (*f).clone()).collect();

        let mut outputs_bigint = Vec::with_capacity(outputs_fr.len());
        for o in outputs_fr.into_iter() {
            let e = gadgets::output::output(self.counter.next(), o.clone())?;
            outputs_bigint.push(e.to_bigint());
        }

        Ok(outputs_bigint)
    }

    pub fn condition_push(&mut self, element: Scalar<E>) -> Result<(), RuntimeError> {
        self.state.conditions_stack.push(element);
        Ok(())
    }

    pub fn condition_pop(&mut self) -> Result<Scalar<E>, RuntimeError> {
        self.state
            .conditions_stack
            .pop()
            .ok_or_else(|| MalformedBytecode::StackUnderflow.into())
    }

    fn top_frame(&mut self) -> Result<&mut Frame<E>, RuntimeError> {
        self.state
            .frames_stack
            .last_mut()
            .ok_or_else(|| MalformedBytecode::StackUnderflow.into())
    }
}

impl<E, CS, S, H> IVirtualMachine for Contract<E, CS, S, H>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
    S: IMerkleTree<E>,
    H: IMerkleTreeHasher<E>,
{
    type E = E;
    type CS = CS;

    fn push(&mut self, cell: Cell<E>) -> Result<(), RuntimeError> {
        self.state.evaluation_stack.push(cell)
    }

    fn pop(&mut self) -> Result<Cell<E>, RuntimeError> {
        self.state.evaluation_stack.pop()
    }

    fn load(&mut self, address: usize) -> Result<Cell<E>, RuntimeError> {
        let frame_start = self.top_frame()?.stack_frame_start;
        self.state.data_stack.get(frame_start + address)
    }

    fn store(&mut self, address: usize, cell: Cell<E>) -> Result<(), RuntimeError> {
        let frame = self.top_frame()?;
        frame.stack_frame_end =
            std::cmp::max(frame.stack_frame_end, frame.stack_frame_start + address + 1);

        let frame_start = frame.stack_frame_start;

        self.state.data_stack.set(frame_start + address, cell)
    }

    fn storage_load(
        &mut self,
        address: Scalar<Self::E>,
        size: usize,
    ) -> Result<Vec<Scalar<Self::E>>, RuntimeError> {
        self.storage.load(self.counter.next(), size, address)
    }

    fn storage_store(
        &mut self,
        address: Scalar<Self::E>,
        values: Vec<Option<Scalar<Self::E>>>,
    ) -> Result<(), RuntimeError> {
        self.storage.store(self.counter.next(), address, values)
    }

    fn loop_begin(&mut self, iterations: usize) -> Result<(), RuntimeError> {
        let frame = self
            .state
            .frames_stack
            .last_mut()
            .ok_or_else(|| RuntimeError::InternalError("Root frame is missing".into()))?;

        frame.blocks.push(Block::Loop(Loop {
            first_instruction_index: self.state.instruction_counter,
            iterations_left: iterations - 1,
        }));

        Ok(())
    }

    fn loop_end(&mut self) -> Result<(), RuntimeError> {
        let frame = self.state.frames_stack.last_mut().unwrap();

        match frame.blocks.pop() {
            Some(Block::Loop(mut loop_block)) => {
                if loop_block.iterations_left != 0 {
                    loop_block.iterations_left -= 1;
                    self.state.instruction_counter = loop_block.first_instruction_index;
                    frame.blocks.push(Block::Loop(loop_block));
                }
                Ok(())
            }
            _ => Err(MalformedBytecode::UnexpectedLoopEnd.into()),
        }
    }

    fn call(&mut self, address: usize, inputs_count: usize) -> Result<(), RuntimeError> {
        let offset = self.top_frame()?.stack_frame_end;
        self.state
            .frames_stack
            .push(Frame::new(offset, self.state.instruction_counter));

        for i in 0..inputs_count {
            let arg = self.pop()?;
            self.store(inputs_count - i - 1, arg)?;
        }

        self.state.instruction_counter = address;
        Ok(())
    }

    fn r#return(&mut self, outputs_count: usize) -> Result<(), RuntimeError> {
        let mut outputs = Vec::new();
        for _ in 0..outputs_count {
            let output = self.pop()?;
            outputs.push(output);
        }

        let frame = self
            .state
            .frames_stack
            .pop()
            .ok_or(MalformedBytecode::StackUnderflow)?;

        self.state.instruction_counter = frame.return_address;

        for p in outputs.into_iter().rev() {
            self.push(p)?;
        }

        Ok(())
    }

    fn branch_then(&mut self) -> Result<(), RuntimeError> {
        let condition = self.pop()?.try_into_value()?;

        let prev = self.condition_top()?;

        let cs = self.constraint_system();
        let next = gadgets::logical::and::and(cs.namespace(|| "branch"), &condition, &prev)?;
        self.state.conditions_stack.push(next);

        let branch = Branch {
            condition,
            is_else: false,
        };

        self.top_frame()?.blocks.push(Block::Branch(branch));

        self.state.evaluation_stack.fork();
        self.state.data_stack.fork();

        Ok(())
    }

    fn branch_else(&mut self) -> Result<(), RuntimeError> {
        let frame = self
            .state
            .frames_stack
            .last_mut()
            .ok_or_else(|| RuntimeError::InternalError("Root frame is missing".into()))?;

        let mut branch = match frame.blocks.pop() {
            Some(Block::Branch(branch)) => Ok(branch),
            Some(_) | None => Err(RuntimeError::MalformedBytecode(
                MalformedBytecode::UnexpectedElse,
            )),
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

        self.state.data_stack.switch_branch()?;
        self.state.evaluation_stack.fork();

        Ok(())
    }

    fn branch_end(&mut self) -> Result<(), RuntimeError> {
        self.condition_pop()?;

        let frame = self
            .state
            .frames_stack
            .last_mut()
            .ok_or_else(|| RuntimeError::InternalError("Root frame is missing".into()))?;

        let branch = match frame.blocks.pop() {
            Some(Block::Branch(branch)) => Ok(branch),
            Some(_) | None => Err(MalformedBytecode::UnexpectedEndIf),
        }?;

        if branch.is_else {
            self.state
                .evaluation_stack
                .merge(self.counter.next(), &branch.condition)?;
        } else {
            self.state.evaluation_stack.revert()?;
        }

        self.state
            .data_stack
            .merge(self.counter.next(), branch.condition)?;

        Ok(())
    }

    fn exit(&mut self, outputs_count: usize) -> Result<(), RuntimeError> {
        for _ in 0..outputs_count {
            let value = self.pop()?.try_into_value()?;
            self.outputs.push(value);
        }
        self.outputs.reverse();

        self.state.instruction_counter = std::usize::MAX;
        Ok(())
    }

    fn call_native<F: INativeCallable<E>>(&mut self, function: F) -> Result<(), RuntimeError> {
        let stack = &mut self.state.evaluation_stack;
        let cs = &mut self.counter.cs;

        function.call(cs.namespace(|| "native function"), stack)
    }

    fn condition_top(&mut self) -> Result<Scalar<E>, RuntimeError> {
        self.state
            .conditions_stack
            .last()
            .map(|e| (*e).clone())
            .ok_or_else(|| MalformedBytecode::StackUnderflow.into())
    }

    fn constraint_system(&mut self) -> &mut CS {
        &mut self.counter.cs
    }

    fn is_debugging(&self) -> bool {
        self.debugging
    }

    fn get_location(&mut self) -> Location {
        self.location.clone()
    }

    fn set_location(&mut self, location: Location) {
        self.location = location;
    }
}
