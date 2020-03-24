//!
//! The expression translation hint.
//!

#[derive(Debug)]
pub enum Hint {
    // runtime
    PlaceExpression,
    ValueExpression,

    // compile time
    TypeExpression,
    PathExpression,
    CompoundTypeMember,
}
