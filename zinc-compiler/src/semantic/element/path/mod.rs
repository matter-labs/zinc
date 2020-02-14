//!
//! The semantic analyzer path element.
//!

use std::fmt;

use crate::lexical::Location;
use crate::syntax::MemberString;

#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    pub location: Location,
    pub elements: Vec<MemberString>,
}

impl Path {
    pub fn new(location: Location, initial: MemberString) -> Self {
        Self {
            location,
            elements: vec![initial],
        }
    }

    pub fn push_element(&mut self, element: MemberString) {
        self.elements.push(element);
    }

    pub fn last(&self) -> &MemberString {
        self.elements
            .last()
            .expect(crate::semantic::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS)
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
