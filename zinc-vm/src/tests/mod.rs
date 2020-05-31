//!
//! The Zinc VM test tools.
//!

#![cfg(test)]

mod overflow;

use colored::Colorize;
use failure::Fail;
use num_bigint::BigInt;
use num_bigint::ToBigInt;

use bellman::pairing::bn256::Bn256;
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::test::TestConstraintSystem;

use zinc_bytecode::Call;
use zinc_bytecode::DataType;
use zinc_bytecode::Instruction;
use zinc_bytecode::InstructionInfo;
use zinc_bytecode::Program;

use crate::core::VMState;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets::contract::storage::StorageGadget;
use crate::gadgets::contract::Sha256Hasher;
use crate::storage::dummy::DummyStorage;

type TestVirtualMachine =
    VMState<Bn256, TestConstraintSystem<Bn256>, DummyStorage<Bn256>, Sha256Hasher>;

fn new_test_constrained_vm() -> TestVirtualMachine {
    let mut cs = TestConstraintSystem::new();
    let storage = DummyStorage::new(4);
    let storage_gadget = StorageGadget::new(cs.namespace(|| "storage"), storage).unwrap();
    TestVirtualMachine::new(cs, storage_gadget, true)
}

fn assert_stack_eq<VM, BI>(vm: &mut VM, expected_stack: &[BI])
where
    VM: VirtualMachine,
    BI: Into<BigInt> + Copy,
{
    for (i, expected) in expected_stack.iter().enumerate() {
        let value = vm
            .pop()
            .expect("expected stack value is missing")
            .try_into_value()
            .expect("expected Cell::Value");

        assert_eq!(
            value.to_bigint(),
            Some(expected.clone().into()),
            "wrong value on stack at position {}",
            i
        );
    }
}

#[derive(Debug, Fail)]
pub enum TestingError {
    #[fail(display = "{}", _0)]
    RuntimeError(RuntimeError),

    #[fail(display = "unconstrained: {}", _0)]
    Unconstrained(String),

    #[fail(display = "unsatisfied")]
    Unsatisfied,
}

pub struct VMTestRunner {
    instructions: Vec<Instruction>,
}

impl VMTestRunner {
    pub fn new() -> Self {
        Self {
            instructions: vec![Call::new(1, 0).wrap()],
        }
    }

    pub fn add<I: InstructionInfo>(&mut self, instruction: I) -> &mut Self {
        self.instructions.push(instruction.wrap());
        self
    }

    pub fn test<T: Into<BigInt> + Copy>(
        &mut self,
        expected_stack: &[T],
    ) -> Result<(), TestingError> {
        let result = self.test_constrained(expected_stack);

        if let Err(error) = &result {
            println!("{}: {}", "error".bold().red(), error)
        }

        result
    }

    fn test_constrained<T: Into<BigInt> + Copy>(
        &mut self,
        expected_stack: &[T],
    ) -> Result<(), TestingError> {
        let mut vm = new_test_constrained_vm();

        let program = Program::new(DataType::Unit, DataType::Unit, self.instructions.clone());

        vm.run(&program, Some(&[]), |_| {}, |_| Ok(()))
            .map_err(TestingError::RuntimeError)?;

        let cs = vm.constraint_system();

        let unconstrained = cs.find_unconstrained();
        let satisfied = cs.is_satisfied();

        assert_stack_eq(&mut vm, expected_stack);

        if !unconstrained.is_empty() {
            Err(TestingError::Unconstrained(unconstrained))
        } else if !satisfied {
            Err(TestingError::Unsatisfied)
        } else {
            Ok(())
        }
    }
}
