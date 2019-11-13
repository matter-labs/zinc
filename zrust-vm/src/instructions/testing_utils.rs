use crate::{VMInstruction, VirtualMachine, ElementOperator, Element, ConstrainedElement, ConstrainedElementOperator};
use franklin_crypto::circuit::test::TestConstraintSystem;
use bellman::pairing::bn256::Bn256;
use num_bigint::BigInt;

type TestElement = ConstrainedElement<Bn256>;
type TestElementOperator = ConstrainedElementOperator<Bn256, TestConstraintSystem<Bn256>>;
type TestInstruction = dyn VMInstruction<TestElement, TestElementOperator>;
type TestVirtualMachine = VirtualMachine<TestElement, TestElementOperator>;

pub fn create_instructions_vec() -> Vec<Box<TestInstruction>>
{
    Vec::new()
}

pub fn create_vm() -> TestVirtualMachine {
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
    for (i, expected) in expected_stack.iter().enumerate() {
        let value = vm.stack_get(i).expect("expected stack value is missing");
        assert_eq!(value.to_bigint(), Some(expected.clone().into()), "wrong value on stack");
    }
}
