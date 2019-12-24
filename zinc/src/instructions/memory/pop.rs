use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{VMInstruction, InternalVM};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::Pop;

impl<E, O> VMInstruction<E, O> for Pop
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        for _ in 0..self.count {
            vm.pop()?.value()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use num_bigint::BigInt;
    use zinc_bytecode::*;

    #[test]
    fn test_pop() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst {
                value: BigInt::from(1),
            })
            .add(PushConst {
                value: BigInt::from(2),
            })
            .add(Pop::new(1))
            .add(PushConst {
                value: BigInt::from(3),
            })
            .add(PushConst {
                value: BigInt::from(4),
            })
            .add(PushConst {
                value: BigInt::from(5),
            })
            .add(Pop::new(2))
            .test(&[3, 1])
    }
}
