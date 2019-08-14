//!
//! The witness builder.
//!

use crate::lexical::Identifier;
use crate::syntax::Type;
use crate::syntax::Witness;

#[derive(Default)]
pub struct Builder {
    identifier: Option<Identifier>,
    r#type: Option<Type>,
}

impl Builder {
    pub fn set_identifier(&mut self, value: Identifier) {
        self.identifier = Some(value);
    }

    pub fn set_type(&mut self, value: Type) {
        self.r#type = Some(value);
    }

    pub fn build(&mut self) -> Witness {
        Witness::new(
            self.identifier.take().expect("Missing identifier"),
            self.r#type.take().expect("Missing type"),
        )
    }
}
