//!
//! The Zinc VM test tools.
//!

use std::collections::HashMap;

use colored::Colorize;
use failure::Fail;
use num::bigint::ToBigInt;
use num::BigInt;

use franklin_crypto::bellman::pairing::bn256::Bn256;
use franklin_crypto::circuit::test::TestConstraintSystem;

use zinc_build::Call;
use zinc_build::Circuit as BuildCircuit;
use zinc_build::Instruction;
use zinc_build::Type as BuildType;

use crate::core::circuit::State;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;

type TestVirtualMachine = State<Bn256, TestConstraintSystem<Bn256>>;

fn new_test_constrained_vm() -> TestVirtualMachine {
    let cs = TestConstraintSystem::new();
    TestVirtualMachine::new(cs, true)
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

#[derive(Debug, Fail)]
pub enum TestingError {
    #[fail(display = "{}", _0)]
    RuntimeError(RuntimeError),

    #[fail(display = "unconstrained: {}", _0)]
    Unconstrained(String),

    #[fail(display = "unsatisfied")]
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

        let circuit = BuildCircuit::new(
            "test".to_owned(),
            0,
            BuildType::Unit,
            BuildType::Unit,
            HashMap::new(),
            self.instructions,
        );

        vm.run(circuit, Some(&[]), |_| {}, |_| Ok(()))
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
