//!
//! The member string builder.
//!

use crate::lexical::Location;
use crate::syntax::tree::member_string::MemberString;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    name: Option<String>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    pub fn finish(mut self) -> MemberString {
        let location = self
            .location
            .take()
            .unwrap_or_else(|| panic!("{}{}", crate::PANIC_BUILDER_REQUIRES_VALUE, "location"));

        let name = self
            .name
            .take()
            .unwrap_or_else(|| panic!("{}{}", crate::PANIC_BUILDER_REQUIRES_VALUE, "name"));

        MemberString::new(location, name)
    }
}
