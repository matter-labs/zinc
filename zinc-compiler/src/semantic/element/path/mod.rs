//!
//! The semantic analyzer path element.
//!

use std::fmt;

use crate::lexical::token::location::Location;
use crate::syntax::tree::identifier::Identifier;

///
/// Paths are the `::` expressions which only exist at compile-time.
/// They are usually coerced to place, value, constant or type expressions.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    pub location: Location,
    pub elements: Vec<Identifier>,
}

impl Path {
    pub fn new(location: Location, initial: Identifier) -> Self {
        Self {
            location,
            elements: vec![initial],
        }
    }

    pub fn push_element(&mut self, element: Identifier) {
        self.elements.push(element);
    }

    pub fn last(&self) -> &Identifier {
        self.elements
            .last()
            .expect(crate::panic::VALIDATED_DURING_SYNTAX_ANALYSIS)
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.elements
                .iter()
                .map(|identifier| identifier.name.as_str())
                .collect::<Vec<&str>>()
                .join("::"),
        )
    }
}
