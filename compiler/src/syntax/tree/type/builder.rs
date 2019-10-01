//!
//! The type builder.
//!

use crate::lexical::IntegerLiteral;
use crate::lexical::Keyword;
use crate::lexical::Location;
use crate::syntax::Type;
use crate::syntax::TypeVariant;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    is_void: bool,
    keyword: Option<Keyword>,
    array_type_variant: Option<TypeVariant>,
    array_size: Option<IntegerLiteral>,
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

    pub fn set_array_type_variant(&mut self, value: TypeVariant) {
        self.array_type_variant = Some(value);
    }

    pub fn set_array_size(&mut self, value: IntegerLiteral) {
        self.array_size = Some(value);
    }

    pub fn finish(mut self) -> Type {
        let location = self.location.take().expect("Missing location");
        let variant = if self.is_void {
            TypeVariant::Void
        } else if let Some(keyword) = self.keyword.take() {
            match keyword {
                Keyword::Bool => TypeVariant::Boolean,
                Keyword::Uint { bitlength } => TypeVariant::uint(bitlength),
                Keyword::Int { bitlength } => TypeVariant::int(bitlength),
                Keyword::Field => TypeVariant::Field,
                _ => panic!("Always is one of the type keywords above"),
            }
        } else if let Some(array_type) = self.array_type_variant.take() {
            let array_size: usize = self.array_size.take().expect("Missing array size").into();
            TypeVariant::array(array_type, array_size)
        } else {
            panic!("Always processed by branches above and never gets here");
        };

        Type { location, variant }
    }
}
