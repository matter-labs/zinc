mod constrained;
pub mod stdlib;
pub mod utils;

use std::fmt::Debug;

use crate::ZincEngine;
use bellman::ConstraintSystem;
use num_bigint::BigInt;

pub use constrained::*;

use crate::gadgets::utils::dummy_constraint_system::DummyConstraintSystem;
use crate::vm::RuntimeError;
use franklin_crypto::bellman::Variable;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DataType {
    pub signed: bool,
    pub length: usize,
}

impl DataType {
    pub const BOOLEAN: Self = DataType {
        signed: false,
        length: 1,
    };
}

/// Primitive is a primitive value that can be stored on the stack and operated by VM's instructions.
#[derive(Clone)]
pub struct Primitive<E: ZincEngine> {
    value: Option<E::Fr>,
    variable: Variable,
    data_type: Option<DataType>,
}

impl<E: ZincEngine> Primitive<E> {
    pub fn get_data_type(&self) -> Option<DataType> {
        self.data_type
    }
}

pub trait Gadget<E: ZincEngine> {
    type Input;
    type Output;

    /// Synthesize circuit for the function.
    fn synthesize<CS: ConstraintSystem<E>>(
        &self,
        cs: CS,
        input: Self::Input,
    ) -> Result<Self::Output, RuntimeError>;

    /// Calculate function's result without synthesizing a circuit.
    fn calculate(&self, input: Self::Input) -> Result<Self::Output, RuntimeError> {
        let cs = DummyConstraintSystem;
        self.synthesize(cs, input)
    }

    fn input_from_vec(input: &[Primitive<E>]) -> Result<Self::Input, RuntimeError>;
    fn output_into_vec(output: Self::Output) -> Vec<Primitive<E>>;

    fn synthesize_vec<CS: ConstraintSystem<E>>(
        &self,
        cs: CS,
        input: &[Primitive<E>],
    ) -> Result<Vec<Primitive<E>>, RuntimeError> {
        let input = Self::input_from_vec(input)?;
        let output = self.synthesize(cs, input)?;
        Ok(Self::output_into_vec(output))
    }
}

/// PrimitiveOperations is an entity that knows how to operate with some Primitive.
pub trait PrimitiveOperations<E: ZincEngine> {
    type E: ZincEngine;
    type CS: ConstraintSystem<Self::E>;

    fn variable_none(&mut self) -> Result<Primitive<E>, RuntimeError>;
    fn variable_bigint(&mut self, value: &BigInt) -> Result<Primitive<E>, RuntimeError>;
    fn constant_bigint(&mut self, value: &BigInt) -> Result<Primitive<E>, RuntimeError>;
    fn constant_bigint_typed(
        &mut self,
        value: &BigInt,
        data_type: DataType,
    ) -> Result<Primitive<E>, RuntimeError>;
    fn output(&mut self, element: Primitive<E>) -> Result<Primitive<E>, RuntimeError>;
    fn set_type(
        &mut self,
        value: Primitive<E>,
        data_type: DataType,
    ) -> Result<Primitive<E>, RuntimeError>;

    fn add(
        &mut self,
        left: Primitive<E>,
        right: Primitive<E>,
    ) -> Result<Primitive<E>, RuntimeError>;
    fn sub(
        &mut self,
        left: Primitive<E>,
        right: Primitive<E>,
    ) -> Result<Primitive<E>, RuntimeError>;
    fn mul(
        &mut self,
        left: Primitive<E>,
        right: Primitive<E>,
    ) -> Result<Primitive<E>, RuntimeError>;
    fn div_rem(
        &mut self,
        left: Primitive<E>,
        right: Primitive<E>,
    ) -> Result<(Primitive<E>, Primitive<E>), RuntimeError>;
    fn neg(&mut self, element: Primitive<E>) -> Result<Primitive<E>, RuntimeError>;

    fn not(&mut self, element: Primitive<E>) -> Result<Primitive<E>, RuntimeError>;
    fn and(
        &mut self,
        left: Primitive<E>,
        right: Primitive<E>,
    ) -> Result<Primitive<E>, RuntimeError>;
    fn or(&mut self, left: Primitive<E>, right: Primitive<E>)
        -> Result<Primitive<E>, RuntimeError>;
    fn xor(
        &mut self,
        left: Primitive<E>,
        right: Primitive<E>,
    ) -> Result<Primitive<E>, RuntimeError>;

    fn lt(&mut self, left: Primitive<E>, right: Primitive<E>)
        -> Result<Primitive<E>, RuntimeError>;
    fn le(&mut self, left: Primitive<E>, right: Primitive<E>)
        -> Result<Primitive<E>, RuntimeError>;
    fn eq(&mut self, left: Primitive<E>, right: Primitive<E>)
        -> Result<Primitive<E>, RuntimeError>;
    fn ne(&mut self, left: Primitive<E>, right: Primitive<E>)
        -> Result<Primitive<E>, RuntimeError>;
    fn ge(&mut self, left: Primitive<E>, right: Primitive<E>)
        -> Result<Primitive<E>, RuntimeError>;
    fn gt(&mut self, left: Primitive<E>, right: Primitive<E>)
        -> Result<Primitive<E>, RuntimeError>;

    fn conditional_select(
        &mut self,
        condition: Primitive<E>,
        if_true: Primitive<E>,
        if_false: Primitive<E>,
    ) -> Result<Primitive<E>, RuntimeError>;
    fn assert(&mut self, element: Primitive<E>) -> Result<(), RuntimeError>;

    fn array_get(
        &mut self,
        array: &[Primitive<E>],
        index: Primitive<E>,
    ) -> Result<Primitive<E>, RuntimeError>;
    fn array_set(
        &mut self,
        array: &[Primitive<E>],
        index: Primitive<E>,
        value: Primitive<E>,
    ) -> Result<Vec<Primitive<E>>, RuntimeError>;

    fn execute<G: Gadget<Self::E>>(
        &mut self,
        gadget: G,
        input: &[Primitive<E>],
    ) -> Result<Vec<Primitive<E>>, RuntimeError>;
}
