mod constrained;
mod simple;
pub mod utils;

pub use constrained::*;
pub use simple::*;

use crate::vm::RuntimeError;
use num_bigint::{BigInt, ToBigInt};
use std::fmt::{Debug, Display};

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
pub trait Primitive: Sized + Clone + Debug + Display + ToBigInt {}

/// PrimitiveOperations is an entity that knows how to operate with some Primitive.
pub trait PrimitiveOperations<P: Primitive> {
    fn variable_none(&mut self) -> Result<P, RuntimeError>;
    fn variable_bigint(&mut self, value: &BigInt) -> Result<P, RuntimeError>;
    fn constant_bigint(&mut self, value: &BigInt) -> Result<P, RuntimeError>;
    fn constant_bigint_typed(
        &mut self,
        value: &BigInt,
        data_type: DataType,
    ) -> Result<P, RuntimeError>;
    fn output(&mut self, element: P) -> Result<P, RuntimeError>;
    fn set_type(&mut self, value: P, data_type: DataType) -> Result<P, RuntimeError>;

    fn add(&mut self, left: P, right: P) -> Result<P, RuntimeError>;
    fn sub(&mut self, left: P, right: P) -> Result<P, RuntimeError>;
    fn mul(&mut self, left: P, right: P) -> Result<P, RuntimeError>;
    fn div_rem(&mut self, left: P, right: P) -> Result<(P, P), RuntimeError>;
    fn neg(&mut self, element: P) -> Result<P, RuntimeError>;

    fn not(&mut self, element: P) -> Result<P, RuntimeError>;
    fn and(&mut self, left: P, right: P) -> Result<P, RuntimeError>;
    fn or(&mut self, left: P, right: P) -> Result<P, RuntimeError>;
    fn xor(&mut self, left: P, right: P) -> Result<P, RuntimeError>;

    fn lt(&mut self, left: P, right: P) -> Result<P, RuntimeError>;
    fn le(&mut self, left: P, right: P) -> Result<P, RuntimeError>;
    fn eq(&mut self, left: P, right: P) -> Result<P, RuntimeError>;
    fn ne(&mut self, left: P, right: P) -> Result<P, RuntimeError>;
    fn ge(&mut self, left: P, right: P) -> Result<P, RuntimeError>;
    fn gt(&mut self, left: P, right: P) -> Result<P, RuntimeError>;

    fn conditional_select(
        &mut self,
        condition: P,
        if_true: P,
        if_false: P,
    ) -> Result<P, RuntimeError>;
    fn assert(&mut self, element: P) -> Result<(), RuntimeError>;

    fn array_get(&mut self, array: &[P], index: P) -> Result<P, RuntimeError>;
    fn array_set(&mut self, array: &[P], index: P, value: P) -> Result<Vec<P>, RuntimeError>;
}
