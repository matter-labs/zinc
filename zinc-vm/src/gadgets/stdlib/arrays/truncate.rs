use crate::errors::MalformedBytecode;
use crate::gadgets::{Gadget, Scalar};
use crate::Engine;
use crate::RuntimeError;
use bellman::ConstraintSystem;

/// Truncate array.
///
/// Signature: fn truncate(len: field, array: [field; len], new_len: field);
pub struct Truncate;

impl<E: Engine> Gadget<E> for Truncate {
    type Input = (Scalar<E>, Vec<Scalar<E>>);
    type Output = Vec<Scalar<E>>;

    fn synthesize<CS: ConstraintSystem<E>>(
        &self,
        _cs: CS,
        input: Self::Input,
    ) -> Result<Self::Output, RuntimeError> {
        let (new_len, mut array) = input;
        let len = new_len.get_constant_usize()?;

        if len > array.len() {
            return Err(MalformedBytecode::InvalidArguments(format!(
                "Truncate: new length ({}) can't be greater than old length ({})",
                len,
                array.len()
            ))
            .into());
        }

        array.truncate(len);

        Ok(array)
    }

    fn input_from_vec(input: &[Scalar<E>]) -> Result<Self::Input, RuntimeError> {
        let (new_len, array) = input.split_last().ok_or_else(|| {
            MalformedBytecode::InvalidArguments("truncate expects at least one argument".into())
        })?;

        Ok((new_len.clone(), Vec::from(array)))
    }

    fn output_into_vec(output: Self::Output) -> Vec<Scalar<E>> {
        output
    }
}
