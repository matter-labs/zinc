use crate::element::{Element, ElementOperator};
use crate::RuntimeError;
use num_bigint::{BigInt, ToBigInt};
use num_traits::ToPrimitive;
use std::fmt::{Debug, Display, Formatter, Error};

/// PrimitiveElement is an Element implementation
/// that uses rust's primitive integer type to represent a value.
///
/// It's purpose is to provide faster computation.
#[derive(Debug, Clone)]
pub struct PrimitiveElement {
    value: u128,
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

impl ElementOperator<PrimitiveElement> for PrimitiveElementOperator {
    fn constant_u64(&mut self, value: u64) -> Result<PrimitiveElement, RuntimeError> {
        Ok(PrimitiveElement {value: value as u128})
    }

    fn constant_bigint(&mut self, value: &BigInt) -> Result<PrimitiveElement, RuntimeError> {
        let v = value.to_u128().ok_or(RuntimeError::IntegerOverflow)?;
        Ok(PrimitiveElement { value: v })
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
        let div = left.value.checked_div(right.value).ok_or(RuntimeError::IntegerOverflow)?;
        let rem = left.value.checked_rem(right.value).ok_or(RuntimeError::IntegerOverflow)?;

        Ok((
           PrimitiveElement { value: div },
           PrimitiveElement { value: rem },
        ))
    }

    fn neg(&mut self, element: PrimitiveElement) -> Result<PrimitiveElement, RuntimeError> {
        let value: u128 = (0 as u128).wrapping_sub(element.value);
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
}
