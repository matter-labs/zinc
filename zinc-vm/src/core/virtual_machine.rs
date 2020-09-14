//!
//! The virtual machine trait.
//!

use franklin_crypto::bellman::ConstraintSystem;

use crate::core::execution_state::cell::Cell;
use crate::core::location::Location;
use crate::error::RuntimeError;
use crate::gadgets::scalar::Scalar;
use crate::instructions::call_std::INativeCallable;
use crate::IEngine;

///
/// This trait represents virtual machine's interface. It is used by instructions.
///
pub trait IVirtualMachine {
    type E: IEngine;
    type CS: ConstraintSystem<Self::E>;

    // Operations with evaluation stack

    fn push(&mut self, cell: Cell<Self::E>) -> Result<(), RuntimeError>;
    fn pop(&mut self) -> Result<Cell<Self::E>, RuntimeError>;

    // Operations with data stack

    fn load(&mut self, address: usize) -> Result<Cell<Self::E>, RuntimeError>;
    fn store(&mut self, address: usize, cell: Cell<Self::E>) -> Result<(), RuntimeError>;

    // Operations with contract storage

    fn storage_load(
        &mut self,
        index: Scalar<Self::E>,
        size: usize,
    ) -> Result<Vec<Scalar<Self::E>>, RuntimeError>;
    fn storage_store(
        &mut self,
        index: Scalar<Self::E>,
        values: Vec<Scalar<Self::E>>,
    ) -> Result<(), RuntimeError>;

    fn loop_begin(&mut self, iter_count: usize) -> Result<(), RuntimeError>;
    fn loop_end(&mut self) -> Result<(), RuntimeError>;

    fn call(&mut self, address: usize, inputs_count: usize) -> Result<(), RuntimeError>;
    fn r#return(&mut self, outputs_count: usize) -> Result<(), RuntimeError>;

    fn branch_then(&mut self) -> Result<(), RuntimeError>;
    fn branch_else(&mut self) -> Result<(), RuntimeError>;
    fn branch_end(&mut self) -> Result<(), RuntimeError>;

    fn exit(&mut self, values_count: usize) -> Result<(), RuntimeError>;

    fn call_native<F: INativeCallable<Self::E>>(&mut self, function: F)
        -> Result<(), RuntimeError>;

    fn condition_top(&mut self) -> Result<Scalar<Self::E>, RuntimeError>;

    fn set_unconstrained(&mut self);
    fn unset_unconstrained(&mut self);
    fn is_unconstrained(&self) -> bool;

    fn constraint_system(&mut self) -> &mut Self::CS;

    fn is_debugging(&self) -> bool;

    fn get_location(&mut self) -> Location;

    fn set_location(&mut self, location: Location);
}
