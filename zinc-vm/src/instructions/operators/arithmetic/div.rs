use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::Div;
use zinc_bytecode::ScalarType;

use crate::core::state::cell::Cell;
use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::gadgets::scalar::scalar_type::ScalarTypeExpectation;
use crate::gadgets::scalar::Scalar;
impl<VM: VirtualMachine> VMInstruction<VM> for Div {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.try_into_value()?;
        let left = vm.pop()?.try_into_value()?;

        let condition = vm.condition_top()?;
        let scalar_type = ScalarType::expect_same(left.get_type(), right.get_type())?;

        let cs = vm.constraint_system();

        let div = match scalar_type {
            ScalarType::Field => {
                let one = Scalar::new_constant_int(1, right.get_type());
                let denom = gadgets::conditional_select::conditional_select(
                    cs.namespace(|| "select denom"),
                    &condition,
                    &right,
                    &one,
                )?;
                let inverse =
                    gadgets::arithmetic::field::inverse(cs.namespace(|| "inverse"), &denom)?;
                gadgets::arithmetic::mul::mul(cs.namespace(|| "div"), &left, &inverse)?
            }
            ScalarType::Integer(_) => {
                let (unchecked_div, _rem) = gadgets::arithmetic::div_rem::div_rem_conditional(
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
    use crate::tests::TestingError;
    use crate::tests::VMTestRunner;

    use zinc_bytecode::IntegerType;

    #[test]
    fn test_div() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(zinc_bytecode::Push::new((9).into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Push::new((4).into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Div)
            .add(zinc_bytecode::Push::new((9).into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Push::new(
                (-4).into(),
                IntegerType::I8.into(),
            ))
            .add(zinc_bytecode::Div)
            .add(zinc_bytecode::Push::new(
                (-9).into(),
                IntegerType::I8.into(),
            ))
            .add(zinc_bytecode::Push::new((4).into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Div)
            .add(zinc_bytecode::Push::new(
                (-9).into(),
                IntegerType::I8.into(),
            ))
            .add(zinc_bytecode::Push::new(
                (-4).into(),
                IntegerType::I8.into(),
            ))
            .add(zinc_bytecode::Div)
            .test(&[3, -3, -2, 2])
    }
}
