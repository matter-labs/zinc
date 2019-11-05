use crate::{Bytecode, Stack, RuntimeError};
use bellman::pairing::Engine;
use franklin_crypto::bellman::ConstraintSystem;

pub trait Operator<E, CS> where E: Engine, CS: ConstraintSystem<E> {
    fn execute(
        &self,
        cs: &mut CS,
        stack: &mut Stack<E>,
        bytecode: &mut Bytecode)
        -> Result<(), RuntimeError>;
}
