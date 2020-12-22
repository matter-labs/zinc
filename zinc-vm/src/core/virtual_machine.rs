//!
//! The virtual machine trait.
//!

use franklin_crypto::bellman::ConstraintSystem;

use crate::core::contract::storage::leaf::LeafVariant;
use crate::core::execution_state::cell::Cell;
use crate::core::location::Location;
use crate::error::Error;
use crate::gadgets::contract::merkle_tree::IMerkleTree;
use crate::gadgets::scalar::Scalar;
use crate::instructions::call_library::INativeCallable;
use crate::IEngine;

///
/// This trait represents virtual machine's interface. It is used by instructions.
///
pub trait IVirtualMachine {
    type E: IEngine;
    type CS: ConstraintSystem<Self::E>;
    type S: IMerkleTree<Self::E>;

    // Operations with evaluation stack

    fn push(&mut self, cell: Cell<Self::E>) -> Result<(), Error>;
    fn pop(&mut self) -> Result<Cell<Self::E>, Error>;

    // Operations with data stack

    fn load(&mut self, address: usize) -> Result<Cell<Self::E>, Error>;
    fn store(&mut self, address: usize, cell: Cell<Self::E>) -> Result<(), Error>;

    // Operations with contract storage

    fn storage_init(
        &mut self,
        project: zinc_project::ManifestProject,
        values: Vec<Scalar<Self::E>>,
        field_types: Vec<zinc_types::ContractFieldType>,
    ) -> Result<Scalar<Self::E>, Error>;
    fn storage_fetch(
        &mut self,
        eth_address: Scalar<Self::E>,
        field_types: Vec<zinc_types::ContractFieldType>,
    ) -> Result<(), Error>;
    fn storage_load(
        &mut self,
        eth_address: Scalar<Self::E>,
        index: Scalar<Self::E>,
        size: usize,
    ) -> Result<Vec<Scalar<Self::E>>, Error>;
    fn storage_store(
        &mut self,
        eth_address: Scalar<Self::E>,
        index: Scalar<Self::E>,
        values: LeafVariant<Self::E>,
    ) -> Result<(), Error>;
    fn storages_count(&self) -> usize;

    // Flow control operations

    fn loop_begin(&mut self, iter_count: usize) -> Result<(), Error>;
    fn loop_end(&mut self) -> Result<(), Error>;

    fn call(&mut self, address: usize, inputs_count: usize) -> Result<(), Error>;
    fn r#return(&mut self, outputs_count: usize) -> Result<(), Error>;

    fn branch_then(&mut self) -> Result<(), Error>;
    fn branch_else(&mut self) -> Result<(), Error>;
    fn branch_end(&mut self) -> Result<(), Error>;

    fn call_native<F: INativeCallable<Self::E, Self::S>>(
        &mut self,
        function: F,
    ) -> Result<(), Error>;

    fn condition_top(&mut self) -> Result<Scalar<Self::E>, Error>;

    fn constraint_system(&mut self) -> &mut Self::CS;

    fn get_location(&mut self) -> Location;

    fn set_location(&mut self, location: Location);
}
