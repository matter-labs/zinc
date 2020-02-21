mod internal;
pub mod location;
mod state;

pub use crate::errors::RuntimeError;
pub use internal::*;
pub use state::*;

use crate::core::location::CodeLocation;
use crate::errors::MalformedBytecode;
use crate::gadgets::{Gadgets, Scalar, ScalarType};
use crate::Engine;
use colored::Colorize;
use franklin_crypto::bellman::ConstraintSystem;
use num_bigint::{BigInt, ToBigInt};
use std::marker::PhantomData;
use zinc_bytecode::data::types as object_types;
use zinc_bytecode::program::Program;
use zinc_bytecode::{dispatch_instruction, Instruction, InstructionInfo};

pub trait VMInstruction<E, CS>: InstructionInfo
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError>;
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

pub struct VirtualMachine<E: Engine, CS: ConstraintSystem<E>> {
    pub(crate) debugging: bool,
    state: State<E>,
    cs: CounterNamespace<E, CS>,
    outputs: Vec<Scalar<E>>,
    pub(crate) location: CodeLocation,
}

impl<E: Engine, CS: ConstraintSystem<E>> VirtualMachine<E, CS> {
    pub fn new(cs: CS, debugging: bool) -> Self {
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
        }
    }

    pub fn constraint_system(&mut self) -> &mut CS {
        &mut self.cs.cs
    }

    pub fn run<CB, F>(
        &mut self,
        program: &Program,
        inputs: Option<&[BigInt]>,
        mut instruction_callback: CB,
        mut check_cs: F,
    ) -> Result<Vec<Option<BigInt>>, RuntimeError>
    where
        CB: FnMut(&CS) -> (),
        F: FnMut(&CS) -> Result<(), RuntimeError>,
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

    fn init_root_frame(
        &mut self,
        input_type: &object_types::DataType,
        inputs: Option<&[BigInt]>,
    ) -> Result<(), RuntimeError> {
        self.state
            .frames_stack
            .push(FunctionFrame::new(0, std::usize::MAX));

        let types = data_type_into_scalar_types(&input_type);

        match inputs {
            None => {
                for t in types {
                    let variable = self.operations().variable_none(t)?;
                    if let ScalarType::Field = t {
                        // Add constraint so circuit doesn't fail if argument is not used.
                        // TODO: Refactor this.
                        self.operations().neg(variable.clone())?;
                    }
                    self.push(Cell::Value(variable))?;
                }
            }
            Some(values) => {
                for (value, dtype) in values.iter().zip(types) {
                    let variable = self.operations().variable_bigint(value, dtype)?;
                    if let ScalarType::Field = dtype {
                        // Add constraint so circuit doesn't fail if argument is not used.
                        // TODO: Refactor this.
                        self.operations().neg(variable.clone())?;
                    }
                    self.push(Cell::Value(variable))?;
                }
            }
        }

        Ok(())
    }

    fn get_outputs(&mut self) -> Result<Vec<Option<BigInt>>, RuntimeError> {
        let outputs_fr: Vec<_> = self.outputs.iter().map(|f| (*f).clone()).collect();

        let mut outputs_bigint = Vec::with_capacity(outputs_fr.len());
        for o in outputs_fr.into_iter() {
            let e = self.operations().output(o.clone())?;
            outputs_bigint.push(e.to_bigint());
        }

        Ok(outputs_bigint)
    }

    pub fn operations(&mut self) -> Gadgets<E, bellman::Namespace<E, CS::Root>> {
        Gadgets::new(self.cs.namespace())
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

    pub fn condition_top(&mut self) -> Result<Scalar<E>, RuntimeError> {
        self.state
            .conditions_stack
            .last()
            .map(|e| (*e).clone())
            .ok_or_else(|| MalformedBytecode::StackUnderflow.into())
    }

    fn top_frame(&mut self) -> Result<&mut FunctionFrame<E>, RuntimeError> {
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
