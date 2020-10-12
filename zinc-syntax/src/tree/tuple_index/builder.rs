//!
//! The tuple index builder.
//!

use zinc_lexical::Location;

use crate::tree::literal::integer::Literal as IntegerLiteral;
use crate::tree::tuple_index::TupleIndex;

///
/// The tuple index builder.
///
#[derive(Default)]
pub struct Builder {
    /// The location of the syntax construction.
    location: Option<Location>,
    /// The tuple index integer literal.
    literal: Option<IntegerLiteral>,
}

impl Builder {
    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_literal(&mut self, value: IntegerLiteral) {
        self.literal = Some(value);
    }

    ///
    /// Finalizes the builder and returns the built value.
    ///
    /// # Panics
    /// If some of the required items has not been set.
    ///
    pub fn finish(mut self) -> TupleIndex {
        let location = self.location.take().unwrap_or_else(|| {
            panic!(
                "{}{}",
                zinc_const::panic::BUILDER_REQUIRES_VALUE,
                "location"
            )
        });

        let literal = self.literal.take().unwrap_or_else(|| {
            panic!("{}{}", zinc_const::panic::BUILDER_REQUIRES_VALUE, "literal")
        });

        TupleIndex::new(location, literal)
    }
}
