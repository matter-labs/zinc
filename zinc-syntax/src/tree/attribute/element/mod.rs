//!
//! The attribute element.
//!

pub mod builder;
pub mod variant;

use zinc_lexical::Location;

use crate::tree::expression::tree::Tree as ExpressionTree;

use self::variant::Variant;

///
/// The attribute element.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    /// The location of the syntax construction.
    pub location: Location,
    /// The attribute identifier.
    pub path: ExpressionTree,
    /// The attribute optional variant.
    pub variant: Option<Variant>,
}

impl Element {
    ///
    /// Creates the attribute value.
    ///
    pub fn new(location: Location, path: ExpressionTree, variant: Option<Variant>) -> Self {
        Self {
            location,
            path,
            variant,
        }
    }
}
