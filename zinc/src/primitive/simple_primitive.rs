use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::RuntimeError;
use num_bigint::{BigInt, ToBigInt};
use num_traits::ToPrimitive;
use std::fmt::{Debug, Display, Error, Formatter};

/// SimplePrimitive is a Primitive implementation
/// that uses rust's primitive integer type to represent a value.
///
/// It's purpose is to provide faster computation.
#[derive(Debug, Clone)]
pub struct SimplePrimitive {
    value: i128,
}

impl Display for SimplePrimitive {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        Display::fmt(&self.value, f)
    }
}

impl ToBigInt for SimplePrimitive {
    fn to_bigint(&self) -> Option<BigInt> {
        Some(self.value.into())
    }
}

impl Primitive for SimplePrimitive {}

pub struct PrimitiveElementOperator {}

impl PrimitiveElementOperator {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {}
    }
}

impl PrimitiveOperations<SimplePrimitive> for PrimitiveElementOperator {
    fn variable_none(&mut self) -> Result<SimplePrimitive, RuntimeError> {
        Ok(SimplePrimitive { value: 0 })
    }

    fn variable_bigint(&mut self, value: &BigInt) -> Result<SimplePrimitive, RuntimeError> {
        self.constant_bigint(value)
    }

    fn constant_bigint(&mut self, value: &BigInt) -> Result<SimplePrimitive, RuntimeError> {
        let value = value.to_i128().ok_or(RuntimeError::IntegerOverflow)?;
        Ok(SimplePrimitive { value })
    }

    fn output(&mut self, element: SimplePrimitive) -> Result<SimplePrimitive, RuntimeError> {
        Ok(element)
    }

    fn add(
        &mut self,
        left: SimplePrimitive,
        right: SimplePrimitive,
    ) -> Result<SimplePrimitive, RuntimeError> {
        let value = left
            .value
            .checked_add(right.value)
            .ok_or(RuntimeError::IntegerOverflow)?;
        Ok(SimplePrimitive { value })
    }

    fn sub(
        &mut self,
        left: SimplePrimitive,
        right: SimplePrimitive,
    ) -> Result<SimplePrimitive, RuntimeError> {
        let value = left
            .value
            .checked_sub(right.value)
            .ok_or(RuntimeError::IntegerOverflow)?;
        Ok(SimplePrimitive { value })
    }

    fn mul(
        &mut self,
        left: SimplePrimitive,
        right: SimplePrimitive,
    ) -> Result<SimplePrimitive, RuntimeError> {
        let value = left
            .value
            .checked_mul(right.value)
            .ok_or(RuntimeError::IntegerOverflow)?;
        Ok(SimplePrimitive { value })
    }

    fn div_rem(
        &mut self,
        left: SimplePrimitive,
        right: SimplePrimitive,
    ) -> Result<(SimplePrimitive, SimplePrimitive), RuntimeError> {
        let nominator = left.value;
        let denominator = right.value;
        let mut div = nominator / denominator;

        if div * denominator > nominator {
            if denominator > 0 {
                div -= 1;
            } else {
                div += 1;
            }
        }

        let rem = nominator - div * denominator;

        Ok((
            SimplePrimitive { value: div },
            SimplePrimitive { value: rem },
        ))
    }

    fn neg(&mut self, element: SimplePrimitive) -> Result<SimplePrimitive, RuntimeError> {
        let value = (0 as i128).wrapping_sub(element.value);
        Ok(SimplePrimitive { value })
    }

    fn not(&mut self, element: SimplePrimitive) -> Result<SimplePrimitive, RuntimeError> {
        let value = 1 - element.value;

        Ok(SimplePrimitive { value })
    }

    fn and(
        &mut self,
        left: SimplePrimitive,
        right: SimplePrimitive,
    ) -> Result<SimplePrimitive, RuntimeError> {
        let value = if left.value != 0 && right.value != 0 {
            1
        } else {
            0
        };

        Ok(SimplePrimitive { value })
    }

    fn or(
        &mut self,
        left: SimplePrimitive,
        right: SimplePrimitive,
    ) -> Result<SimplePrimitive, RuntimeError> {
        let value = if left.value != 0 || right.value != 0 {
            1
        } else {
            0
        };

        Ok(SimplePrimitive { value })
    }

    fn xor(
        &mut self,
        left: SimplePrimitive,
        right: SimplePrimitive,
    ) -> Result<SimplePrimitive, RuntimeError> {
        let value = if (left.value != 0) != (right.value != 0) {
            1
        } else {
            0
        };

        Ok(SimplePrimitive { value })
    }

    fn lt(
        &mut self,
        left: SimplePrimitive,
        right: SimplePrimitive,
    ) -> Result<SimplePrimitive, RuntimeError> {
        let value = if left.value < right.value { 1 } else { 0 };

        Ok(SimplePrimitive { value })
    }

    fn le(
        &mut self,
        left: SimplePrimitive,
        right: SimplePrimitive,
    ) -> Result<SimplePrimitive, RuntimeError> {
        let value = if left.value <= right.value { 1 } else { 0 };

        Ok(SimplePrimitive { value })
    }

    fn eq(
        &mut self,
        left: SimplePrimitive,
        right: SimplePrimitive,
    ) -> Result<SimplePrimitive, RuntimeError> {
        let value = if left.value == right.value { 1 } else { 0 };

        Ok(SimplePrimitive { value })
    }

    fn ne(
        &mut self,
        left: SimplePrimitive,
        right: SimplePrimitive,
    ) -> Result<SimplePrimitive, RuntimeError> {
        let value = if left.value != right.value { 1 } else { 0 };

        Ok(SimplePrimitive { value })
    }

    fn ge(
        &mut self,
        left: SimplePrimitive,
        right: SimplePrimitive,
    ) -> Result<SimplePrimitive, RuntimeError> {
        let value = if left.value >= right.value { 1 } else { 0 };

        Ok(SimplePrimitive { value })
    }

    fn gt(
        &mut self,
        left: SimplePrimitive,
        right: SimplePrimitive,
    ) -> Result<SimplePrimitive, RuntimeError> {
        let value = if left.value > right.value { 1 } else { 0 };

        Ok(SimplePrimitive { value })
    }

    fn conditional_select(
        &mut self,
        condition: SimplePrimitive,
        if_true: SimplePrimitive,
        if_false: SimplePrimitive,
    ) -> Result<SimplePrimitive, RuntimeError> {
        let value = if condition.value != 0 {
            if_true.value
        } else {
            if_false.value
        };

        Ok(SimplePrimitive { value })
    }

    fn assert(&mut self, element: SimplePrimitive) -> Result<(), RuntimeError> {
        match element.value {
            0 => Err(RuntimeError::AssertionError),
            _ => Ok(()),
        }
    }

    fn array_get(&mut self, array: &[SimplePrimitive], index: SimplePrimitive) -> Result<SimplePrimitive, RuntimeError> {
        let i = index.value;
        array
            .get(i as usize)
            .map(|p| (*p).clone())
            .ok_or(RuntimeError::IndexOutOfBounds)
    }

    fn array_set(&mut self, array: &[SimplePrimitive], index: SimplePrimitive, value: SimplePrimitive) -> Result<Vec<SimplePrimitive>, RuntimeError> {
        let mut array = Vec::from(array);
        let i = index.value as usize;
        if i >= array.len() {
            Err(RuntimeError::IndexOutOfBounds)
        } else {
            array[i] = value;
            Ok(array)
        }
    }
}
