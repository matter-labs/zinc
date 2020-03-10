extern crate franklin_crypto;

use self::franklin_crypto::bellman::ConstraintSystem;
use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::gadgets;
use crate::gadgets::{Scalar, ScalarType, ScalarTypeExpectation};
use crate::Engine;
use zinc_bytecode::instructions::Div;

impl<E, CS> VMInstruction<E, CS> for Div
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let condition = vm.condition_top()?;
        let scalar_type = ScalarType::expect_same(left.get_type(), right.get_type())?;

        let cs = vm.constraint_system();

        let div = match scalar_type {
            ScalarType::Field => {
                let one = Scalar::new_constant_int(1, right.get_type());
                let denom = gadgets::conditional_select(
                    cs.namespace(|| "select denom"),
                    &condition,
                    &right,
                    &one,
                )?;
                let inverse = gadgets::inverse(vm.constraint_system(), &denom)?;
                vm.operations().mul(left, inverse)?
            }
            ScalarType::Integer(_) => {
                let (unchecked_div, _rem) = gadgets::div_rem_conditional(
                    cs.namespace(|| "div_rem_conditional"),
                    &condition,
                    &left,
                    &right,
                )?;

                gadgets::types::conditional_type_check(
                    cs.namespace(|| "type check"),
                    &condition,
                    &unchecked_div,
                    scalar_type,
                )?
            }
            _ => {
                return Err(RuntimeError::TypeError {
                    expected: "integer or field".to_string(),
                    actual: scalar_type.to_string(),
                })
            }
        };

        vm.push(Cell::Value(div))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::scalar::IntegerType;
    use zinc_bytecode::*;

    #[test]
    fn test_div() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new((9).into(), IntegerType::I8.into()))
            .add(PushConst::new((4).into(), IntegerType::I8.into()))
            .add(Div)
            .add(PushConst::new((9).into(), IntegerType::I8.into()))
            .add(PushConst::new((-4).into(), IntegerType::I8.into()))
            .add(Div)
            .add(PushConst::new((-9).into(), IntegerType::I8.into()))
            .add(PushConst::new((4).into(), IntegerType::I8.into()))
            .add(Div)
            .add(PushConst::new((-9).into(), IntegerType::I8.into()))
            .add(PushConst::new((-4).into(), IntegerType::I8.into()))
            .add(Div)
            .test(&[3, -3, -2, 2])
    }
}
