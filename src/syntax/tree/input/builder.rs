//!
//! The input builder.
//!

use crate::lexical::Identifier;
use crate::syntax::Input;
use crate::syntax::Type;

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

    pub fn build(&mut self) -> Input {
        Input::new(
            self.identifier.take().expect("Missing identifier"),
            self.r#type.take().expect("Missing type"),
        )
    }
}
