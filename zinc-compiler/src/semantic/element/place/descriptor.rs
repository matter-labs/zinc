//!
//! The semantic analyzer place element descriptor.
//!

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Descriptor {
    ArrayIndex(usize),
    TupleField(usize),
    StructureField(String),
}
