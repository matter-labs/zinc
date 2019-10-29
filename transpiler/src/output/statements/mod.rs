//!
//! Transpiler output statements.
//!

mod r#enum;
mod r#let;
mod r#loop;
mod r#struct;
mod r#type;

pub use self::r#enum::Output as EnumOutput;
pub use self::r#let::Output as LetOutput;
pub use self::r#loop::OutputFor as LoopForOutput;
pub use self::r#loop::OutputWhile as LoopWhileOutput;
pub use self::r#struct::Output as StructOutput;
pub use self::r#type::Output as TypeOutput;
