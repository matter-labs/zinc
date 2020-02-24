pub mod prelude {
    pub use crate::gadgets::{Scalar, ScalarVariant};
    pub use crate::constraint_systems::ConstantCS;
}

///
/// # Example
/// ```
/// ```
#[macro_export]
macro_rules! auto_const {
    // Unary operators
    ($op:path, $cs:expr, $a:expr) => {{
        let a = $a;
        match a.get_variant() {
            ScalarVariant::Constant { .. } => {
                println!("constant");
                let const_cs = ConstantCS;
                let result = $op(const_cs, a);
                result.and_then(|scalar| scalar.as_constant_unchecked())
            }
            _ => {
                println!("variable");
                $op($cs, a)
            }
        }
    }};
    // Binary operators
    ($op:path, $cs:expr, $a:expr, $b:expr) => {{
        let a = $a;
        let b = $b;
        match (a.get_variant(), b.get_variant()) {
            (ScalarVariant::Constant { .. }, ScalarVariant::Constant { .. }) => {
                println!("constant");
                let const_cs = ConstantCS;
                let result = $op(const_cs, a, b);
                result.and_then(|scalar| scalar.as_constant_unchecked())
            }
            _ => {
                println!("variable");
                $op($cs, a, b)
            }
        }
    }};
}
