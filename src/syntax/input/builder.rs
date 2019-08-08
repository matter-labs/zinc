//!
//! The syntax input builder.
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
        Input {
            identifier: match self.identifier.take() {
                Some(identifier) => identifier,
                None => panic!("Missing identifier"),
            },
            r#type: match self.r#type.take() {
                Some(r#type) => r#type,
                None => panic!("Missing type"),
            },
        }
    }
}
