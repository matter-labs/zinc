use std::rc::Rc;
use std::collections::HashMap;
use bellman::pairing::Engine;
use franklin_crypto::bellman::ConstraintSystem;
use crate::{Stack, VMInstruction};
use zrust_bytecode::InstructionCode;

#[derive(Debug)]
pub enum RuntimeError {
    InvalidOperation(u8),
    InvalidArguments,
    StackUnderflow,
    StackOverflow,
    UnexpectedEndOfFile,
    SynthesisError,
    InternalError,
}

pub struct VirtualMachine<E> where E: Engine {
    stack: Stack<E>,
}

impl<E> VirtualMachine<E> where E: Engine {
    pub fn new() -> Self {
        Self { stack: Stack::new() }
    }

    pub fn run<CS>(&mut self, cs: &mut CS, instructions: &[Box<dyn VMInstruction<E, CS>>])
        -> Result<(), RuntimeError>
    where
        CS: ConstraintSystem<E>
    {
        for (i, instr) in instructions.iter().enumerate() {
            cs.push_namespace(|| format!("{}", i));

            log::info!(">>> {}", instr.to_assembly());
            instr.execute(cs, &mut self.stack)?;
            self.log_stack();

            cs.pop_namespace();
        }
        Ok(())
    }

    pub fn log_stack(&self) {
        for i in 0..self.stack.len() {
            if i > 10 {
                break;
            }
            match self.stack.get(i) {
                None => log::info!("none"),
                Some(p) => {
                    match p.value {
                        None => log::info!("none"),
                        Some(fr) => log::info!("{:?}", fr),
                    }
                }
            }
        }
    }

    pub fn stack(&self) -> &Stack<E> {
        &self.stack
    }
}
