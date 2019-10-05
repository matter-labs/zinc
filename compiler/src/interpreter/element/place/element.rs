//!
//! The interpreter place element.
//!

#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    ArrayIndex(usize),
    TupleField(usize),
    StructureField(String),
}
