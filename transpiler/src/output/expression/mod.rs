//!
//! Transpiler output complex expressions.
//!

mod array;
mod block;
mod conditional;
mod structure;
mod tuple;

pub use self::array::Output as ArrayOutput;
pub use self::block::Output as BlockOutput;
pub use self::conditional::Output as ConditionalOutput;
pub use self::structure::Output as StructureOutput;
pub use self::tuple::Output as TupleOutput;
