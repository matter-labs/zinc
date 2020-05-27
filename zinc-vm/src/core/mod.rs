mod internal;
pub mod location;
mod state;

pub use crate::error::RuntimeError;
pub use internal::*;
pub use state::*;

use crate::core::location::CodeLocation;
use crate::error::MalformedBytecode;
use crate::gadgets::{Gadgets, Scalar};
use crate::Engine;
use colored::Colorize;
use franklin_crypto::bellman::ConstraintSystem;
use num_bigint::{BigInt, ToBigInt};
use std::marker::PhantomData;
use zinc_bytecode::DataType;
use zinc_bytecode::Program;
use zinc_bytecode::ScalarType;
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
        input_type: &DataType,
        inputs: Option<&[BigInt]>,
    ) -> Result<(), RuntimeError> {
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

fn data_type_into_scalar_types(dtype: &DataType) -> Vec<ScalarType> {
    fn internal(types: &mut Vec<ScalarType>, dtype: &DataType) {
        match dtype {
            DataType::Unit => {}
            DataType::Scalar(scalar_type) => {
                types.push(scalar_type.to_owned());
            }
            DataType::Enum => {
                types.push(ScalarType::Field);
            }
            DataType::Struct(fields) => {
                for (_, r#type) in fields {
                    internal(types, r#type);
                }
            }
            DataType::Tuple(fields) => {
                for r#type in fields {
                    internal(types, r#type);
                }
            }
            DataType::Array(r#type, size) => {
                for _ in 0..*size {
                    internal(types, r#type.as_ref());
                }
            }
        }
    }

    let mut types = Vec::new();
    internal(&mut types, dtype);
    types
}
