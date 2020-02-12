use crate::core::{InternalVM, RuntimeError, VirtualMachine};
use crate::Engine;
use bellman::pairing::bn256::Bn256;
use colored::Colorize;
use failure::Fail;
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::test::TestConstraintSystem;
use num_bigint::{BigInt, ToBigInt};
use zinc_bytecode::data::types::DataType;
use zinc_bytecode::{Call, Instruction, InstructionInfo, Program};

type TestVirtualMachine = VirtualMachine<Bn256, TestConstraintSystem<Bn256>>;

fn new_test_constrained_vm() -> TestVirtualMachine {
    let cs = TestConstraintSystem::new();
    TestVirtualMachine::new(cs, true)
}

fn assert_stack_eq<E, CS, BI>(vm: &mut VirtualMachine<E, CS>, expected_stack: &[BI])
where
    E: Engine,
    CS: ConstraintSystem<E>,
    BI: Into<BigInt> + Copy,
{
    for (i, expected) in expected_stack.iter().enumerate() {
        let value = vm
            .pop()
            .expect("expected stack value is missing")
            .value()
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
