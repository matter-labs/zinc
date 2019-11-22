use crate::element::{Element, ElementOperator};
use num_bigint::{BigInt, ToBigInt};
use num_traits::ToPrimitive;
use std::fmt::{Debug, Display, Formatter, Error};
use crate::vm::RuntimeError;

/// PrimitiveElement is an Element implementation
/// that uses rust's primitive integer type to represent a value.
///
/// It's purpose is to provide faster computation.
#[derive(Debug, Clone)]
pub struct PrimitiveElement {
    value: i128,
}

impl Display for PrimitiveElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        Display::fmt(&self.value, f)
    }
}

impl ToBigInt for PrimitiveElement {
    fn to_bigint(&self) -> Option<BigInt> {
        Some(self.value.into())
    }
}

impl Element for PrimitiveElement {}

pub struct PrimitiveElementOperator {}

impl PrimitiveElementOperator {
    pub fn new() -> Self {
        Self {}
    }
}

impl ElementOperator<PrimitiveElement> for PrimitiveElementOperator {
    fn input_bigint(&mut self, value: &BigInt) -> Result<PrimitiveElement, RuntimeError> {
        self.constant_bigint(value)
    }

    fn constant_bigint(&mut self, value: &BigInt) -> Result<PrimitiveElement, RuntimeError> {
        let value = value.to_i128().ok_or(RuntimeError::IntegerOverflow)?;
        Ok(PrimitiveElement { value })
    }

    fn add(&mut self, left: PrimitiveElement, right: PrimitiveElement) -> Result<PrimitiveElement, RuntimeError> {
        let value = left.value.checked_add(right.value).ok_or(RuntimeError::IntegerOverflow)?;
        Ok(PrimitiveElement { value })
    }

    fn sub(&mut self, left: PrimitiveElement, right: PrimitiveElement) -> Result<PrimitiveElement, RuntimeError> {
        let value = left.value.checked_add(right.value).ok_or(RuntimeError::IntegerOverflow)?;
        Ok(PrimitiveElement { value })
    }

    fn mul(&mut self, left: PrimitiveElement, right: PrimitiveElement) -> Result<PrimitiveElement, RuntimeError> {
        let value = left.value.checked_mul(right.value).ok_or(RuntimeError::IntegerOverflow)?;
        Ok(PrimitiveElement { value })

    }

    fn div_rem(&mut self, left: PrimitiveElement, right: PrimitiveElement)
        -> Result<(PrimitiveElement, PrimitiveElement), RuntimeError>
    {
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
           PrimitiveElement { value: div },
           PrimitiveElement { value: rem },
        ))
    }

    fn neg(&mut self, element: PrimitiveElement) -> Result<PrimitiveElement, RuntimeError> {
        let value = (0 as i128).wrapping_sub(element.value);
        Ok(PrimitiveElement { value })
    }

    fn not(&mut self, element: PrimitiveElement) -> Result<PrimitiveElement, RuntimeError> {
        let value = 1 - element.value;

        Ok(PrimitiveElement { value })
    }

    fn and(&mut self, left: PrimitiveElement, right: PrimitiveElement) -> Result<PrimitiveElement, RuntimeError> {
        let value = if left.value != 0 && right.value != 0 { 1 } else { 0 };

        Ok(PrimitiveElement { value })
    }

    fn or(&mut self, left: PrimitiveElement, right: PrimitiveElement) -> Result<PrimitiveElement, RuntimeError> {
        let value = if left.value != 0 || right.value != 0 { 1 } else { 0 };

        Ok(PrimitiveElement { value })
    }

    fn xor(&mut self, left: PrimitiveElement, right: PrimitiveElement) -> Result<PrimitiveElement, RuntimeError> {
        let value = if (left.value != 0) != (right.value != 0) { 1 } else { 0 };

        Ok(PrimitiveElement { value })
    }

    fn lt(&mut self, left: PrimitiveElement, right: PrimitiveElement) -> Result<PrimitiveElement, RuntimeError> {
        let value = if left.value < right.value { 1 } else { 0 };

        Ok(PrimitiveElement { value })
    }

    fn le(&mut self, left: PrimitiveElement, right: PrimitiveElement) -> Result<PrimitiveElement, RuntimeError> {
        let value = if left.value <= right.value { 1 } else { 0 };

        Ok(PrimitiveElement { value })
    }

    fn eq(&mut self, left: PrimitiveElement, right: PrimitiveElement) -> Result<PrimitiveElement, RuntimeError> {
        let value = if left.value == right.value { 1 } else { 0 };

        Ok(PrimitiveElement { value })
    }

    fn ne(&mut self, left: PrimitiveElement, right: PrimitiveElement) -> Result<PrimitiveElement, RuntimeError> {
        let value = if left.value != right.value { 1 } else { 0 };

        Ok(PrimitiveElement { value })
    }

    fn ge(&mut self, left: PrimitiveElement, right: PrimitiveElement) -> Result<PrimitiveElement, RuntimeError> {
        let value = if left.value >= right.value { 1 } else { 0 };

        Ok(PrimitiveElement { value })
    }

    fn gt(&mut self, left: PrimitiveElement, right: PrimitiveElement) -> Result<PrimitiveElement, RuntimeError> {
        let value = if left.value > right.value { 1 } else { 0 };

        Ok(PrimitiveElement { value })
    }

    fn conditional_select(&mut self, condition: PrimitiveElement, if_true: PrimitiveElement, if_false: PrimitiveElement) -> Result<PrimitiveElement, RuntimeError> {
        let value = if condition.value != 0 { if_true.value } else { if_false.value };

        Ok(PrimitiveElement { value })
    }

    fn assert(&mut self, element: PrimitiveElement) -> Result<(), RuntimeError> {
        match element.value {
            0 => Err(RuntimeError::AssertionError),
            _ => Ok(())
        }
    }
}
