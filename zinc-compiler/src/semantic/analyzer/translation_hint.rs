//!
//! The semantic translation hint.
//!

#[derive(Debug)]
pub enum TranslationHint {
    ValueExpression,
    TypeExpression,

    PathExpression,
    PlaceExpression,
    CompoundTypeMember,
}
