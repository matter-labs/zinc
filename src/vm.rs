use crate::stack::Stack;
use std::io;
use std::collections::HashMap;
use franklin_crypto::bellman::{Variable, ConstraintSystem};
use bellman::pairing::Engine;
use crate::opcodes::OpCode;
use crate::operators;
use std::rc::Rc;

pub enum RuntimeError {
    InvalidOperation,
    StackUnderflow,
    StackOverflow,
    IOError(io::Error),
    UnexpectedEndOfFile,
    InternalError,
}

pub trait Operator<E, CS> where E: Engine, CS: ConstraintSystem<E> {
    fn execute(
            &self,
            cs: &mut CS,
            stack: &mut Stack<Variable>,
            bytecode: &mut dyn io::Read)
       -> Result<(), RuntimeError>;
}

pub struct VirtualMachine<E, CS> where E: Engine, CS: ConstraintSystem<E> {
    stack: Stack<Variable>,
    opcodes: HashMap<u8, Rc<Box<dyn Operator<E, CS>>>>,
}

impl<E, CS> VirtualMachine<E, CS> where E: Engine, CS: ConstraintSystem<E> {
    pub fn new() -> Self {
        let mut vm = Self {
            stack: Stack::new(),
            opcodes: HashMap::new(),
        };

        vm.opcodes.insert(OpCode::Push as u8, Rc::new(Box::new(operators::Push)));
        vm.opcodes.insert(OpCode::Pop as u8, Rc::new(Box::new(operators::Pop)));

        vm
    }

    pub fn run<R: io::Read>(&mut self, cs: &mut CS, bytecode: &mut R) -> Result<(), RuntimeError> {
        let mut bytes: [u8; 1] = [0];
        loop {
            let code: u8;
            match bytecode.read(&mut bytes) {
                Ok(1) => { code = bytes[0] },
                Ok(_) => return Ok(()),
                Err(e) => return Err(RuntimeError::IOError(e)),
            }

            let operator = self.dispatch(code)?;
            operator.execute(
                cs,
                &mut self.stack,
                bytecode)?;
        }
    }

    fn dispatch(&self, code: u8) -> Result<Rc<Box<dyn Operator<E, CS>>>, RuntimeError> {
        match self.opcodes.get(&code) {
            None => Err(RuntimeError::InvalidOperation),
            Some(op) => Ok(op.clone()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use franklin_crypto::circuit::test::TestConstraintSystem;
    use bellman::pairing::bn256::Bn256;

    #[test]
    fn test_vm() {
        let mut cs = TestConstraintSystem::<Bn256>::new();
        let mut vm = VirtualMachine::<Bn256, TestConstraintSystem<Bn256>>::new();
        let mut bytecode: &[u8] = &[0x01, 0x02];

        match vm.run(&mut cs, &mut bytecode) {
            Ok(_) => {},
            Err(_) => {},
        }
    }
}
