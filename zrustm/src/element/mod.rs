mod primitive_element;
mod constrained_element;
pub mod utils;

pub use primitive_element::*;
pub use constrained_element::*;

use crate::vm::RuntimeError;
use num_bigint::{BigInt, ToBigInt};
use std::fmt::{Debug, Display};

/// Element is a primitive value that can be stored on the stack and operated by VM's instructions.
pub trait Element: Sized + Clone + Debug + Display + ToBigInt {}

/// ElementOperator is an entity that knows how to operate on Element.
pub trait ElementOperator<E: Element> {
    fn variable_none(&mut self) -> Result<E, RuntimeError>;
    fn variable_bigint(&mut self, value: &BigInt) -> Result<E, RuntimeError>;
    fn constant_bigint(&mut self, value: &BigInt) -> Result<E, RuntimeError>;
    fn output(&mut self, element: E) -> Result<E, RuntimeError>;

    fn add(&mut self, left: E, right: E) -> Result<E, RuntimeError>;
    fn sub(&mut self, left: E, right: E) -> Result<E, RuntimeError>;
    fn mul(&mut self, left: E, right: E) -> Result<E, RuntimeError>;
    fn div_rem(&mut self, left: E, right: E) -> Result<(E, E), RuntimeError>;
    fn neg(&mut self, element: E) -> Result<E, RuntimeError>;

    fn not(&mut self, element: E) -> Result<E, RuntimeError>;
    fn and(&mut self, left: E, right: E) -> Result<E, RuntimeError>;
    fn or(&mut self, left: E, right: E) -> Result<E, RuntimeError>;
    fn xor(&mut self, left: E, right: E) -> Result<E, RuntimeError>;

    fn lt(&mut self, left: E, right: E) -> Result<E, RuntimeError>;
    fn le(&mut self, left: E, right: E) -> Result<E, RuntimeError>;
    fn eq(&mut self, left: E, right: E) -> Result<E, RuntimeError>;
    fn ne(&mut self, left: E, right: E) -> Result<E, RuntimeError>;
    fn ge(&mut self, left: E, right: E) -> Result<E, RuntimeError>;
    fn gt(&mut self, left: E, right: E) -> Result<E, RuntimeError>;

    fn conditional_select(&mut self, condition: E, if_true: E, if_false: E) -> Result<E, RuntimeError>;
    fn assert(&mut self, element: E) -> Result<(), RuntimeError>;
}
