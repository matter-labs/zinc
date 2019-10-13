//!
//! Transpiler output allocation.
//!

mod boolean;
mod number_const;
mod number_index;

pub use self::boolean::Output as BooleanOutput;
pub use self::number_const::Output as NumberConstantOutput;
pub use self::number_index::Output as NumberIndexOutput;
