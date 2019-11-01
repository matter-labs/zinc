use crate::stack::Stack;
use std::io;
use std::collections::HashMap;
use franklin_crypto::bellman::{Variable, ConstraintSystem, Index};
use bellman::pairing::Engine;
use crate::opcodes::OpCode;
use crate::operators;
use std::rc::Rc;
use franklin_crypto::circuit::test::TestConstraintSystem;

#[derive(Debug)]
pub enum RuntimeError {
    InvalidOperation(u8),
    InvalidArguments,
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

    pub fn execute_one<R: io::Read>(&mut self, cs: &mut CS, bytecode: &mut R) -> Result<bool, RuntimeError> {
        let code: u8;
        let mut bytes: [u8; 1] = [0];
        match bytecode.read(&mut bytes) {
            Ok(1) => { code = bytes[0] },
            Ok(_) => return Ok(false),
            Err(e) => return Err(RuntimeError::IOError(e)),
        }

        let operator = self.dispatch(code)?;

        dbg!(code);

        operator.execute(
            cs,
            &mut self.stack,
            bytecode)?;

        Ok(true)
    }

    pub fn run<R: io::Read>(&mut self, cs: &mut CS, bytecode: &mut R) -> Result<(), RuntimeError> {
        let mut i = 0;
        loop {
            cs.push_namespace(|| format!("{}", i));
            if !self.execute_one(cs, bytecode)? {
                break;
            }
            cs.pop_namespace();
            i += 1;
        }
        Ok(())
    }

    fn dispatch(&self, code: u8) -> Result<Rc<Box<dyn Operator<E, CS>>>, RuntimeError> {
        match self.opcodes.get(&code) {
            None => Err(RuntimeError::InvalidOperation(code)),
            Some(op) => Ok(op.clone()),
        }
    }
}

impl<E: Engine> VirtualMachine<E, TestConstraintSystem<E>> {
    pub fn log_stack(&self, cs: &mut TestConstraintSystem<E>) {
        println!(">>> stack");
        for i in 0..self.stack.len() {
            match self.stack.get(i) {
                None => println!("none"),
                Some(var) => {
                    match var.get_unchecked() {
                        Index::Input(_) => println!("input"),
                        Index::Aux(index) => println!("{}", cs.aux[index].0)
                    }
                },
            }
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
        let mut bytecode: &[u8] = &[
            OpCode::Push as u8, 0x01, 0xAA,
            OpCode::Push as u8, 0x02, 0xBB, 0xBB,
            OpCode::Pop as u8,
            OpCode::Push as u8, 0x02, 0xCC, 0xCC,
        ];

        match vm.run(&mut cs, &mut bytecode) {
            Ok(_) => {},
            Err(e) => {assert!(false, "runtime error: {:?}", e)},
        }

        vm.log_stack(&mut cs);
    }
}
