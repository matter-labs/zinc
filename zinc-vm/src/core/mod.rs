pub mod location;
mod state;

pub use crate::errors::RuntimeError;
pub use state::*;

use crate::core::location::CodeLocation;
use crate::errors::MalformedBytecode;
use crate::gadgets::contracts::merkle_tree_storage::merkle_tree_hash::{
    MerkleTreeHasher, Sha256Hasher,
};
use crate::gadgets::contracts::merkle_tree_storage::MerkleTreeStorage;
use crate::gadgets::{Gadgets, Scalar, ScalarType, StorageGadget};
use crate::stdlib::NativeFunction;
use crate::{gadgets, Engine, Result};
use colored::Colorize;
use franklin_crypto::bellman::ConstraintSystem;
use num_bigint::{BigInt, ToBigInt};
use std::marker::PhantomData;
use zinc_bytecode::data::types as object_types;
use zinc_bytecode::program::Program;
use zinc_bytecode::{dispatch_instruction, Instruction, InstructionInfo};

pub trait VMInstruction<VM: VirtualMachine>: InstructionInfo {
    fn execute(&self, vm: &mut VM) -> Result;
}

struct CounterNamespace<E: Engine, CS: ConstraintSystem<E>> {
    cs: CS,
    counter: usize,
    _pd: PhantomData<E>,
}

impl<E: Engine, CS: ConstraintSystem<E>> CounterNamespace<E, CS> {
    fn new(cs: CS) -> Self {
        Self {
            cs,
            counter: 0,
            _pd: PhantomData,
        }
    }

    fn namespace(&mut self) -> bellman::Namespace<E, CS::Root> {
        let namespace = self.counter.to_string();
        self.counter += 1;
        self.cs.namespace(|| namespace)
    }
}

/// This trait represents virtual machine's interface. It is used by instructions.
pub trait VirtualMachine {
    type E: Engine;
    type CS: ConstraintSystem<Self::E>;

    // Operations with stack

    fn push(&mut self, cell: Cell<Self::E>) -> Result;
    fn pop(&mut self) -> Result<Cell<Self::E>>;

    // Operations with memory

    fn load(&mut self, address: usize) -> Result<Cell<Self::E>>;
    fn load_global(&mut self, address: usize) -> Result<Cell<Self::E>>;
    fn store(&mut self, address: usize, cell: Cell<Self::E>) -> Result;
    fn store_global(&mut self, address: usize, cell: Cell<Self::E>) -> Result;

    // Operations with contracts' storage

    fn storage_load(
        &mut self,
        address: &Scalar<Self::E>,
        size: usize,
    ) -> Result<Vec<Scalar<Self::E>>>;
    fn storage_store(&mut self, address: &Scalar<Self::E>, value: &[Scalar<Self::E>]) -> Result;

    fn loop_begin(&mut self, iter_count: usize) -> Result;
    fn loop_end(&mut self) -> Result;

    fn call(&mut self, address: usize, inputs_count: usize) -> Result;
    fn ret(&mut self, outputs_count: usize) -> Result;

    fn branch_then(&mut self) -> Result;
    fn branch_else(&mut self) -> Result;
    fn branch_end(&mut self) -> Result;

    fn exit(&mut self, values_count: usize) -> Result;
    fn call_native<F: NativeFunction<Self::E>>(&mut self, function: F) -> Result;

    fn condition_top(&mut self) -> Result<Scalar<Self::E>>;

    fn constraint_system(&mut self) -> &mut Self::CS;
    fn operations(
        &mut self,
    ) -> Gadgets<
        Self::E,
        bellman::Namespace<
            Self::E,
            <<Self as VirtualMachine>::CS as ConstraintSystem<Self::E>>::Root,
        >,
    >;
    fn is_debugging(&self) -> bool;
    fn get_location(&mut self) -> CodeLocation;
    fn set_location(&mut self, location: CodeLocation);
}

pub struct VMState<E, CS, S, H>
where
    E: Engine,
    CS: ConstraintSystem<E>,
    S: MerkleTreeStorage<E>,
    H: MerkleTreeHasher<E>,
{
    pub(crate) debugging: bool,
    state: State<E>,
    cs: CounterNamespace<E, CS>,
    outputs: Vec<Scalar<E>>,
    pub(crate) location: CodeLocation,
    storage: StorageGadget<E, S, H>,
}

impl<E, CS, S, H> VMState<E, CS, S, H>
where
    E: Engine,
    CS: ConstraintSystem<E>,
    S: MerkleTreeStorage<E>,
    H: MerkleTreeHasher<E>,
{
    pub fn new(mut cs: CS, debugging: bool, storage: StorageGadget<E, S, H>) -> Self {
        Self {
            debugging,
            state: State {
                instruction_counter: 0,
                evaluation_stack: EvaluationStack::new(),
                data_stack: DataStack::new(),
                conditions_stack: vec![],
                frames_stack: vec![],
            },
            cs: CounterNamespace::new(cs),
            outputs: vec![],
            location: CodeLocation::new(),
            storage,
        }
    }

    pub fn run<CB, F>(
        &mut self,
        program: &Program,
        inputs: Option<&[BigInt]>,
        mut instruction_callback: CB,
        mut check_cs: F,
    ) -> Result<Vec<Option<BigInt>>>
    where
        CB: FnMut(&CS) -> (),
        F: FnMut(&CS) -> Result,
    {
        self.cs.cs.enforce(
            || "ONE * ONE = ONE (do this to avoid `unconstrained` error)",
            |zero| zero + CS::one(),
            |zero| zero + CS::one(),
            |zero| zero + CS::one(),
        );
        let one = self
            .operations()
            .constant_bigint(&1.into(), ScalarType::Boolean)?;
        self.condition_push(one)?;

        self.init_root_frame(&program.input, inputs)?;
        self.init_storage()?;

        let mut step = 0;
        while self.state.instruction_counter < program.bytecode.len() {
            let namespace = format!("step={}, addr={}", step, self.state.instruction_counter);
            self.cs.cs.push_namespace(|| namespace);
            let instruction = &program.bytecode[self.state.instruction_counter];
            log::info!(
                "{}:{} > {}",
                step,
                self.state.instruction_counter,
                dispatch_instruction!(instruction => instruction.to_assembly())
            );
            self.state.instruction_counter += 1;
            let result = dispatch_instruction!(instruction => instruction.execute(self));
            if let Err(err) = result.and(check_cs(&self.cs.cs)) {
                log::error!("{}\nat {}", err, self.location.to_string().blue());
                return Err(err);
            }

            log::trace!("{}", self.state);
            instruction_callback(&self.cs.cs);
            self.cs.cs.pop_namespace();
            step += 1;
        }

        self.get_outputs()
    }

    fn init_storage(&mut self) -> Result {
        // TODO: add root_hash to public input

        // Temporary fix to avoid "unconstrained" error
        let root_hash = self.storage.root_hash()?;
        let cs = self.constraint_system();

        gadgets::add(
            cs.namespace(|| "root_hash constraint"),
            &root_hash,
            &Scalar::new_constant_int(1, ScalarType::Field),
        )?;

        Ok(())
    }

    fn init_root_frame(
        &mut self,
        input_type: &object_types::DataType,
        inputs: Option<&[BigInt]>,
    ) -> Result {
        self.state
            .frames_stack
            .push(FunctionFrame::new(0, std::usize::MAX));

        let types = data_type_into_scalar_types(&input_type);

        // Convert Option<&[BigInt]> to iterator of Option<&BigInt> and zip with types.
        let value_type_pairs: Vec<_> = match inputs {
            Some(values) => values.iter().map(Some).zip(types).collect(),
            None => std::iter::repeat(None).zip(types).collect(),
        };

        for (value, dtype) in value_type_pairs {
            let variable = self.operations().allocate_witness(value, dtype)?;
            self.push(Cell::Value(variable))?;
        }

        Ok(())
    }

    fn get_outputs(&mut self) -> Result<Vec<Option<BigInt>>> {
        let outputs_fr: Vec<_> = self.outputs.iter().map(|f| (*f).clone()).collect();

        let mut outputs_bigint = Vec::with_capacity(outputs_fr.len());
        for o in outputs_fr.into_iter() {
            let e = self.operations().output(o.clone())?;
            outputs_bigint.push(e.to_bigint());
        }

        Ok(outputs_bigint)
    }

    pub fn condition_push(&mut self, element: Scalar<E>) -> Result {
        self.state.conditions_stack.push(element);
        Ok(())
    }

    pub fn condition_pop(&mut self) -> Result<Scalar<E>> {
        self.state
            .conditions_stack
            .pop()
            .ok_or_else(|| MalformedBytecode::StackUnderflow.into())
    }

    fn top_frame(&mut self) -> Result<&mut FunctionFrame<E>> {
        self.state
            .frames_stack
            .last_mut()
            .ok_or_else(|| MalformedBytecode::StackUnderflow.into())
    }
}

fn data_type_into_scalar_types(dtype: &object_types::DataType) -> Vec<ScalarType> {
    fn internal(types: &mut Vec<ScalarType>, dtype: &object_types::DataType) {
        match dtype {
            object_types::DataType::Unit => {}
            object_types::DataType::Scalar(scalar_type) => {
                types.push(*scalar_type);
            }
            object_types::DataType::Enum => {
                types.push(ScalarType::Field);
            }
            object_types::DataType::Struct(fields) => {
                for (_, t) in fields {
                    internal(types, t);
                }
            }
            object_types::DataType::Tuple(fields) => {
                for t in fields {
                    internal(types, t);
                }
            }
            object_types::DataType::Array(t, size) => {
                for _ in 0..*size {
                    internal(types, t.as_ref());
                }
            }
        }
    }

    let mut types = Vec::new();
    internal(&mut types, dtype);
    types
}

impl<E, CS, S, H> VirtualMachine for VMState<E, CS, S, H>
where
    E: Engine,
    CS: ConstraintSystem<E>,
    S: MerkleTreeStorage<E>,
    H: MerkleTreeHasher<E>,
{
    type E = E;
    type CS = CS;

    fn push(&mut self, cell: Cell<E>) -> Result {
        self.state.evaluation_stack.push(cell)
    }

    fn pop(&mut self) -> Result<Cell<E>> {
        self.state.evaluation_stack.pop()
    }

    fn load(&mut self, address: usize) -> Result<Cell<E>> {
        let offset = self.top_frame()?.stack_frame_begin;
        self.state.data_stack.get(offset + address)
    }

    fn load_global(&mut self, address: usize) -> Result<Cell<E>> {
        self.state.data_stack.get(address)
    }

    fn store(&mut self, address: usize, cell: Cell<E>) -> Result {
        {
            let frame = self.top_frame()?;
            frame.stack_frame_end =
                std::cmp::max(frame.stack_frame_end, frame.stack_frame_begin + address + 1);
        }
        let offset = self.top_frame()?.stack_frame_begin;
        self.state.data_stack.set(offset + address, cell)
    }

    fn store_global(&mut self, address: usize, cell: Cell<E>) -> Result {
        self.state.data_stack.set(address, cell)
    }

    fn storage_load(
        &mut self,
        address: &Scalar<Self::E>,
        size: usize,
    ) -> Result<Vec<Scalar<Self::E>>> {
        self.storage.load(self.cs.namespace(), size, address)
    }

    fn storage_store(&mut self, address: &Scalar<Self::E>, value: &[Scalar<Self::E>]) -> Result {
        self.storage.store(self.cs.namespace(), address, value)
    }

    fn loop_begin(&mut self, iterations: usize) -> Result {
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

    fn loop_end(&mut self) -> Result {
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

    fn call(&mut self, address: usize, inputs_count: usize) -> Result {
        let offset = self.top_frame()?.stack_frame_end;
        self.state
            .frames_stack
            .push(FunctionFrame::new(offset, self.state.instruction_counter));

        for i in 0..inputs_count {
            let arg = self.pop()?;
            self.store(inputs_count - i - 1, arg)?;
        }

        self.state.instruction_counter = address;
        Ok(())
    }

    fn ret(&mut self, outputs_count: usize) -> Result {
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

    fn branch_then(&mut self) -> Result {
        let condition = self.pop()?.value()?;

        let prev = self.condition_top()?;

        let cs = self.constraint_system();
        let next = gadgets::boolean::and(cs.namespace(|| "branch"), &condition, &prev)?;
        self.state.conditions_stack.push(next);

        let branch = Branch {
            condition,
            is_full: false,
        };

        self.top_frame()?.blocks.push(Block::Branch(branch));

        self.state.evaluation_stack.fork();
        self.state.data_stack.fork();

        Ok(())
    }

    fn branch_else(&mut self) -> Result {
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

        if branch.is_full {
            return Err(MalformedBytecode::UnexpectedElse.into());
        } else {
            branch.is_full = true;
        }

        let condition = branch.condition.clone();

        frame.blocks.push(Block::Branch(branch));

        self.condition_pop()?;
        let prev = self.condition_top()?;
        let cs = self.constraint_system();
        let not_cond = gadgets::not(cs.namespace(|| "not"), &condition)?;
        let next = self.operations().and(prev, not_cond)?;
        self.condition_push(next)?;

        self.state.data_stack.switch_branch()?;
        self.state.evaluation_stack.fork();

        Ok(())
    }

    fn branch_end(&mut self) -> Result {
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

        if branch.is_full {
            self.state
                .evaluation_stack
                .merge(self.cs.namespace(), &branch.condition)?;
        } else {
            self.state.evaluation_stack.revert()?;
        }

        self.state
            .data_stack
            .merge(branch.condition, &mut Gadgets::new(self.cs.namespace()))?;

        Ok(())
    }

    fn exit(&mut self, outputs_count: usize) -> Result {
        for _ in 0..outputs_count {
            let value = self.pop()?.value()?;
            self.outputs.push(value);
        }
        self.outputs.reverse();

        self.state.instruction_counter = std::usize::MAX;
        Ok(())
    }

    fn call_native<F: NativeFunction<E>>(&mut self, function: F) -> Result {
        let stack = &mut self.state.evaluation_stack;
        let cs = &mut self.cs.cs;

        function.execute(cs.namespace(|| "native function"), stack)
    }

    fn condition_top(&mut self) -> Result<Scalar<E>> {
        self.state
            .conditions_stack
            .last()
            .map(|e| (*e).clone())
            .ok_or_else(|| MalformedBytecode::StackUnderflow.into())
    }

    fn constraint_system(&mut self) -> &mut CS {
        &mut self.cs.cs
    }

    fn operations(&mut self) -> Gadgets<E, bellman::Namespace<E, CS::Root>> {
        Gadgets::new(self.cs.namespace())
    }

    fn is_debugging(&self) -> bool {
        self.debugging
    }

    fn get_location(&mut self) -> CodeLocation {
        self.location.clone()
    }

    fn set_location(&mut self, location: CodeLocation) {
        self.location = location;
    }
}
