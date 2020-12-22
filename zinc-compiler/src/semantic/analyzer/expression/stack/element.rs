//!
//! The expression semantic analyzer stack element.
//!

use zinc_syntax::ExpressionOperand;

use crate::semantic::element::Element as SemanticElement;

///
/// The expression semantic analyzer stack element.
///
#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum Element {
    /// The syntax-level item, which has not been processed by the semantic analyzer yet.
    NotEvaluated(ExpressionOperand),
    /// The semantic-level item, which has already been processed by the semantic analyzer.
    Evaluated(SemanticElement),
}
