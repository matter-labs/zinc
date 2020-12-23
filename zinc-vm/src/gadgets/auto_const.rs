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
