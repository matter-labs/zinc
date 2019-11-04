//!
//! The semantic analyzer element place descriptor.
//!

#[derive(Debug, Clone, PartialEq)]
pub enum Descriptor {
    ArrayIndex(usize),
    TupleField(usize),
    StructureField(String),
}
