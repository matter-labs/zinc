//!
//! The constant optimizing macro.
//!

pub mod prelude {
    pub use crate::constraint_systems::constant::Constant as ConstantCS;
    pub use crate::gadgets::scalar::variant::Variant as ScalarVariant;
    pub use crate::gadgets::scalar::Scalar;

    use crate::error::Error;
    use crate::IEngine;

    pub trait ToConstant: Sized {
        fn to_constant(&self) -> Result<Self, Error>;
    }

    impl<E: IEngine> ToConstant for Scalar<E> {
        fn to_constant(&self) -> Result<Self, Error> {
            self.to_constant_unchecked()
        }
    }

    impl<E: IEngine> ToConstant for (Scalar<E>, Scalar<E>) {
        fn to_constant(&self) -> Result<Self, Error> {
            Ok((
                self.0.to_constant_unchecked()?,
                self.1.to_constant_unchecked()?,
            ))
        }
    }
}

///
/// # Example
/// ```
/// use franklin_crypto::bellman::pairing::ff::PrimeField;
/// use franklin_crypto::bellman::pairing::bn256::Bn256;
/// use franklin_crypto::bellman::pairing::bn256::Fr;
/// use franklin_crypto::circuit::test::TestConstraintSystem;
/// use franklin_crypto::bellman::ConstraintSystem;
///
/// use zinc_vm::auto_const;
/// use zinc_vm::gadgets::scalar::Scalar;
/// use zinc_vm::gadgets;
/// use zinc_vm::gadgets::auto_const::prelude::*;
///
/// let a = Scalar::<Bn256>::new_constant_fr(Fr::from_str("42").expect(zinc_const::panic::TEST_DATA_VALID), zinc_build::ScalarType::Field);
/// let b = Scalar::<Bn256>::new_constant_fr(Fr::from_str("69").expect(zinc_const::panic::TEST_DATA_VALID), zinc_build::ScalarType::Field);
///
/// let mut cs = TestConstraintSystem::<Bn256>::new();
///
/// let c: Scalar<Bn256> = auto_const!(gadgets::arithmetic::mul::mul, cs.namespace(|| "mul"), &a, &b).expect(zinc_const::panic::TEST_DATA_VALID);
///
/// assert!(c.is_constant());
///
/// let expected = Fr::from_str(&(42 * 69).to_string());
/// assert_eq!(c.get_value(), expected);
///
/// assert!(cs.constraints.is_empty())
/// ```
#[macro_export]
macro_rules! auto_const {
    // Unary operators
    ($op:path, $cs:expr, $a:expr) => {{
        let a = $a;
        match a.get_variant() {
            ScalarVariant::Constant { .. } => {
                let const_cs = ConstantCS::default();
                let result = $op(const_cs, a);
                result.and_then(|result| result.to_constant())
            }
            _ => $op($cs, a),
        }
    }};
    // Binary operators
    ($op:path, $cs:expr, $a:expr, $b:expr) => {{
        let a = $a;
        let b = $b;
        match (a.get_variant(), b.get_variant()) {
            (ScalarVariant::Constant { .. }, ScalarVariant::Constant { .. }) => {
                let const_cs = ConstantCS::default();
                let result = $op(const_cs, a, b);
                result.and_then(|result| result.to_constant())
            }
            _ => $op($cs, a, b),
        }
    }};
}
