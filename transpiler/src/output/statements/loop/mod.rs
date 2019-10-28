//!
//! Transpiler output loop.
//!

mod r#for;
mod r#while;

pub use self::r#for::Output as OutputFor;
pub use self::r#while::Output as OutputWhile;
