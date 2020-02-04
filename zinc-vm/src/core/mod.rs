mod internal;
pub mod location;
mod state;

pub use crate::errors::RuntimeError;
pub use internal::*;
pub use state::*;

use crate::core::location::CodeLocation;
use crate::gadgets::{Gadgets, Primitive, ScalarType};
use crate::Engine;
use colored::Colorize;
use franklin_crypto::bellman::ConstraintSystem;
use num_bigint::{BigInt, ToBigInt};
use std::marker::PhantomData;
use zinc_bytecode::data::types::{DataType, PrimitiveType};
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
    outputs: Vec<Primitive<E>>,
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
        let one = self
            .operations()
            .constant_bigint_typed(&1.into(), ScalarType::BOOLEAN)?;
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
                let msg = if let RuntimeError::UnsatisfiedConstraint = err {
                    String::from("value overflow or constraint violation")
                } else {
                    format!("{:?}", err)
                };

                println!(
                    "{} {:?}\n\tat {}",
                    "Error".bold().red(),
                    msg,
                    self.location.to_string().blue()
                );
                return Err(err);
            }

            log::trace!("{}", self.state_to_string());
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

        match inputs {
            None => {
                for t in types {
                    let variable = self.operations().variable_none(t)?;
                    self.push(Cell::Value(variable))?;
                }
            }
            Some(values) => {
                for (value, dtype) in values.iter().zip(types) {
                    let variable = self.operations().variable_bigint(value, dtype)?;
                    self.push(Cell::Value(variable))?;
                }
            }
        }

        Ok(())
    }

    fn get_outputs(&mut self) -> Result<Vec<Option<BigInt>>, RuntimeError> {
        let outputs_fr: Vec<_> = self.outputs.iter().rev().map(|f| (*f).clone()).collect();

        let mut outputs_bigint = Vec::with_capacity(outputs_fr.len());
        for o in outputs_fr.into_iter() {
            let e = self.operations().output(o.clone())?;
            outputs_bigint.push(e.to_bigint());
        }

        Ok(outputs_bigint)
    }

    fn state_to_string(&self) -> String {
        format!("{:#?}", self.state)
    }

    pub fn operations(&mut self) -> Gadgets<E, bellman::Namespace<E, CS::Root>> {
        Gadgets::new(self.cs.namespace())
    }

    pub fn condition_push(&mut self, element: Primitive<E>) -> Result<(), RuntimeError> {
        self.state.conditions_stack.push(element);
        Ok(())
    }

    pub fn condition_pop(&mut self) -> Result<Primitive<E>, RuntimeError> {
        self.state
            .conditions_stack
            .pop()
            .ok_or(RuntimeError::StackUnderflow)
    }

    pub fn condition_top(&mut self) -> Result<Primitive<E>, RuntimeError> {
        self.state
            .conditions_stack
            .last()
            .map(|e| (*e).clone())
            .ok_or(RuntimeError::StackUnderflow)
    }

    fn top_frame(&mut self) -> Result<&mut FunctionFrame<E>, RuntimeError> {
        self.state
            .frames_stack
            .last_mut()
            .ok_or(RuntimeError::StackUnderflow)
    }
}

fn data_type_into_scalar_types(dtype: &DataType) -> Vec<Option<ScalarType>> {
    fn internal(types: &mut Vec<Option<ScalarType>>, dtype: &DataType) {
        match dtype {
            DataType::Unit => {}
            DataType::Primitive(t) => match t {
                PrimitiveType::Field => {
                    types.push(None);
                }
                PrimitiveType::Integer(int) => types.push(Some(ScalarType {
                    signed: int.is_signed,
                    length: int.bit_length,
                })),
            },
            DataType::Enum => {
                types.push(None);
            }
            DataType::Struct(fields) => {
                for (_, t) in fields {
                    internal(types, t);
                }
            }
            DataType::Tuple(fields) => {
                for t in fields {
                    internal(types, t);
                }
            }
            DataType::Array(t, size) => {
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
