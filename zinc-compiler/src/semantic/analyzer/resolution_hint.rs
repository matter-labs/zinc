//!
//! The identifier resolution hint.
//!

#[derive(Debug)]
pub enum ResolutionHint {
    ValueExpression,
    PlaceExpression,
    TypeExpression,
    CompoundTypeMember,
}
