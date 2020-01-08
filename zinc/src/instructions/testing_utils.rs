use crate::primitive::{
    ConstrainingFrOperations, FrPrimitive, Primitive, PrimitiveOperations,
    SimplePrimitiveOperations,
};
use crate::vm::{InternalVM, RuntimeError, VirtualMachine};
use bellman::pairing::bn256::Bn256;
use franklin_crypto::circuit::test::TestConstraintSystem;
use num_bigint::BigInt;
use zinc_bytecode::{decode_all_instructions, Call, DecodingError, InstructionInfo};

type TestElement = FrPrimitive<Bn256>;
type TestElementOperator = ConstrainingFrOperations<Bn256, TestConstraintSystem<Bn256>>;
type TestVirtualMachine = VirtualMachine<TestElement, TestElementOperator>;

fn new_test_constrained_vm() -> TestVirtualMachine {
    let cs = TestConstraintSystem::<Bn256>::new();
    let op = TestElementOperator::new(cs);

    TestVirtualMachine::new(op)
}

fn assert_stack_eq<E, O, BI>(vm: &mut VirtualMachine<E, O>, expected_stack: &[BI])
where
    E: Primitive,
    O: PrimitiveOperations<E>,
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

#[derive(Debug)]
pub enum TestingError {
    DecodingError(DecodingError),
    RuntimeError(RuntimeError),
    Unconstrained(String),
    Unsatisfied,
}

pub struct VMTestRunner {
    bytecode: Vec<u8>,
}

impl VMTestRunner {
    pub fn new() -> Self {
        Self {
            bytecode: Call::new(1, 0).encode(),
        }
    }

    pub fn add<I: InstructionInfo>(&mut self, instruction: I) -> &mut Self {
        self.bytecode.append(&mut instruction.encode());
        self
    }

    pub fn test<T: Into<BigInt> + Copy>(
        &mut self,
        expected_stack: &[T],
    ) -> Result<(), TestingError> {
        self.test_primitive(expected_stack)?;
        self.test_constrained(expected_stack)?;

        Ok(())
    }

    fn test_primitive<T: Into<BigInt> + Copy>(
        &mut self,
        expected_stack: &[T],
    ) -> Result<(), TestingError> {
        let mut instructions = decode_all_instructions(self.bytecode.as_slice())
            .map_err(TestingError::DecodingError)?;

        let mut vm = VirtualMachine::new(SimplePrimitiveOperations::new());

        vm.run(instructions.as_mut_slice(), Some(&[]))
            .map_err(TestingError::RuntimeError)?;

        assert_stack_eq(&mut vm, expected_stack);

        Ok(())
    }

    fn test_constrained<T: Into<BigInt> + Copy>(
        &mut self,
        expected_stack: &[T],
    ) -> Result<(), TestingError> {
        let mut instructions = decode_all_instructions(self.bytecode.as_slice())
            .map_err(TestingError::DecodingError)?;

        let mut vm = new_test_constrained_vm();

        vm.run(instructions.as_mut_slice(), Some(&[]))
            .map_err(TestingError::RuntimeError)?;

        let cs = vm.operations().constraint_system();

        let unconstrained = cs.find_unconstrained();
        let satisfied = cs.is_satisfied();

        assert_stack_eq(&mut vm, expected_stack);

        if unconstrained != "" {
            Err(TestingError::Unconstrained(unconstrained))
        } else if !satisfied {
            Err(TestingError::Unsatisfied)
        } else {
            Ok(())
        }
    }
}
