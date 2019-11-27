//!
//! The identifier builder.
//!

use crate::lexical::Location;
use crate::syntax::Identifier;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    name: Option<String>,
    is_instruction: bool,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    pub fn set_is_instruction(&mut self) {
        self.is_instruction = true;
    }

    pub fn finish(mut self) -> Identifier {
        let location = self.location.take().unwrap_or_else(|| {
            panic!(
                "{}{}",
                crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                "location"
            )
        });

        let name = self.name.take().unwrap_or_else(|| {
            panic!(
                "{}{}",
                crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                "name"
            )
        });

        let is_instruction = self.is_instruction;

        Identifier { location, name, is_instruction }
    }
}
