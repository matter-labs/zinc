//!
//! The semantic translation hint.
//!

#[derive(Debug)]
pub enum TranslationHint {
    // runtime
    PlaceExpression,
    ValueExpression,

    // compile time
    TypeExpression,
    PathExpression,
    CompoundTypeMember,
}
