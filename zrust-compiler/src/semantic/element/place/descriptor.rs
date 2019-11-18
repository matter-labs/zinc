//!
//! The semantic analyzer element place descriptor.
//!

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Descriptor {
    ArrayIndex(usize),
    TupleField(usize),
    StructureField(String),
}
