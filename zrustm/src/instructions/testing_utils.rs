use crate::{VMInstruction, VirtualMachine, ElementOperator, Element, ConstrainedElement, ConstrainedElementOperator, RuntimeError, decode_all_vm_instructions, PrimitiveElement, PrimitiveElementOperator};
use franklin_crypto::circuit::test::TestConstraintSystem;
use bellman::pairing::bn256::Bn256;
use num_bigint::BigInt;
use zrust_bytecode::{Instruction, DecodingError};

type TestElement = ConstrainedElement<Bn256>;
type TestElementOperator = ConstrainedElementOperator<Bn256, TestConstraintSystem<Bn256>>;
type TestInstruction = dyn VMInstruction<TestElement, TestElementOperator>;
type TestVirtualMachine = VirtualMachine<TestElement, TestElementOperator>;

pub fn create_instructions_vec() -> Vec<Box<TestInstruction>>
{
    Vec::new()
}

pub fn new_test_constrained_vm() -> TestVirtualMachine {
    let cs = TestConstraintSystem::<Bn256>::new();
    let op = TestElementOperator::new(cs);

    TestVirtualMachine::new(op)
}

pub fn assert_stack_eq<E, O, BI>(vm: &VirtualMachine<E, O>, expected_stack: &[BI])
where
    E: Element,
    O: ElementOperator<E>,
    BI: Into<BigInt> + Copy,
{
    for (i, expected) in expected_stack.iter().rev().enumerate() {
        let value = vm.stack_get(i).expect("expected stack value is missing");
        assert_eq!(
            value.to_bigint(),
            Some(expected.clone().into()),
            "wrong value on stack at position {}", i
        );
    }
}

#[derive(Debug,PartialEq)]
pub enum TestingError {
    DecodingError(DecodingError),
    RuntimeError(RuntimeError),
    Unconstrained(String),
    Unsatisfied,
}

pub struct VMTestRunner {
    bytecode: Vec<u8>
}

impl VMTestRunner {
    pub fn new() -> Self {
        Self { bytecode: vec![] }
    }

    pub fn add<I: Instruction>(&mut self, instruction: I) -> &mut Self {
        self.bytecode.append(&mut instruction.encode());
        self
    }

    pub fn test<T: Into<BigInt> + Copy>(&mut self, expected_stack: &[T]) -> Result<(), TestingError> {
        self.test_primitive(expected_stack)?;
        self.test_constrained(expected_stack)?;

        Ok(())
    }

    fn test_primitive<T: Into<BigInt> + Copy>(&mut self, expected_stack: &[T]) -> Result<(), TestingError> {
        let mut instructions = decode_all_vm_instructions
            ::<PrimitiveElement, PrimitiveElementOperator>(self.bytecode.as_slice())
            .map_err(|e| TestingError::DecodingError(e))?;

        let mut vm = VirtualMachine::new(PrimitiveElementOperator::new());

        vm
            .run(instructions.as_mut_slice())
            .map_err(|e| TestingError::RuntimeError(e))?;

        assert_stack_eq(&vm, expected_stack);

        Ok(())
    }

    fn test_constrained<T: Into<BigInt> + Copy>(&mut self, expected_stack: &[T]) -> Result<(), TestingError> {
        let mut instructions = decode_all_vm_instructions
            ::<TestElement, TestElementOperator>(self.bytecode.as_slice())
            .map_err(|e| TestingError::DecodingError(e))?;

        let mut vm = new_test_constrained_vm();

        vm
            .run(instructions.as_mut_slice())
            .map_err(|e| TestingError::RuntimeError(e))?;


        let cs = vm.get_operator().constraint_system();

        let unconstrained = cs.find_unconstrained();
        let satisfied = cs.is_satisfied();

        assert_stack_eq(&vm, expected_stack);

        if unconstrained != "" {
            Err(TestingError::Unconstrained(unconstrained))
        } else if !satisfied {
            Err(TestingError::Unsatisfied)
        } else {
            Ok(())
        }
    }
}
