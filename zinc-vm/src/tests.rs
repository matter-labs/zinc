//!
//! The test tools.
//!

use std::collections::HashMap;

use colored::Colorize;
use num::bigint::ToBigInt;
use num::BigInt;
use thiserror::Error;

use franklin_crypto::bellman::pairing::bn256::Bn256;
use franklin_crypto::circuit::test::TestConstraintSystem;

use zinc_types::Call;
use zinc_types::Instruction;

use crate::core::circuit::State;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;

type TestVirtualMachine = State<Bn256, TestConstraintSystem<Bn256>>;

fn new_test_constrained_vm() -> TestVirtualMachine {
    let cs = TestConstraintSystem::new();
    TestVirtualMachine::new(cs)
}

fn assert_stack_eq<VM, BI>(vm: &mut VM, expected_stack: &[BI])
where
    VM: IVirtualMachine,
    BI: Into<BigInt> + Copy,
{
    for (i, expected) in expected_stack.iter().enumerate() {
        let value = vm
            .pop()
            .expect(zinc_const::panic::TEST_DATA_VALID)
            .try_into_value()
            .expect(zinc_const::panic::TEST_DATA_VALID);

        assert_eq!(
            value.to_bigint(),
            Some(expected.clone().into()),
            "wrong value on stack at position {}",
            i
        );
    }
}

#[derive(Debug, Error)]
pub enum TestingError {
    #[error("{0}")]
    Error(Error),

    #[error("unconstrained: {0}")]
    Unconstrained(String),

    #[error("unsatisfied")]
    Unsatisfied,
}

pub struct TestRunner {
    instructions: Vec<Instruction>,
}

impl TestRunner {
    pub fn new() -> Self {
        Self {
            instructions: vec![Call::new(1, 0).into()],
        }
    }

    pub fn push<I: Into<Instruction>>(mut self, instruction: I) -> Self {
        self.instructions.push(instruction.into());
        self
    }

    pub fn test<T: Into<BigInt> + Copy>(self, expected_stack: &[T]) -> Result<(), TestingError> {
        self.test_constrained(expected_stack).map_err(|error| {
            println!("{}: {}", "error".bold().red(), error);
            error
        })
    }

    fn test_constrained<T: Into<BigInt> + Copy>(
        self,
        expected_stack: &[T],
    ) -> Result<(), TestingError> {
        let mut vm = new_test_constrained_vm();

        let circuit = zinc_types::Circuit::new(
            "test".to_owned(),
            0,
            zinc_types::Type::Unit,
            zinc_types::Type::Unit,
            HashMap::new(),
            self.instructions,
        );

        vm.run(circuit, Some(&[]), |_| {}, |_| Ok(()))
            .map_err(TestingError::Error)?;

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
