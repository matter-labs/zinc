use crate::{Bytecode, Stack, RuntimeError};
use bellman::pairing::Engine;
use franklin_crypto::bellman::ConstraintSystem;
use std::fmt::Debug;

pub trait Operator<E, CS>: Debug where E: Engine, CS: ConstraintSystem<E> {
    fn execute(
        &self,
        cs: &mut CS,
        stack: &mut Stack<E>,
        bytecode: &mut Bytecode)
        -> Result<(), RuntimeError>;
}
