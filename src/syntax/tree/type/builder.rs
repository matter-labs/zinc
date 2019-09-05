//!
//! The type builder.
//!

use crate::lexical::Keyword;
use crate::lexical::Location;
use crate::syntax::Type;
use crate::syntax::TypeVariant;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    is_void: bool,
    keyword: Option<Keyword>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_void(&mut self) {
        self.is_void = true;
    }

    pub fn set_keyword(&mut self, value: Keyword) {
        self.keyword = Some(value);
    }

    pub fn finish(self) -> Type {
        let location = self.location.expect("Missing location");
        let variant = if self.is_void {
            TypeVariant::Void
        } else {
            match self.keyword {
                Some(Keyword::Bool) => TypeVariant::Bool,
                Some(Keyword::Uint { bitlength }) => TypeVariant::uint(bitlength),
                Some(Keyword::Int { bitlength }) => TypeVariant::int(bitlength),
                Some(Keyword::Field) => TypeVariant::Field,
                _ => panic!("The keyword does not describe a type"),
            }
        };

        Type { location, variant }
    }
}
