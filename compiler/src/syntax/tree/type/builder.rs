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
    is_unit: bool,
    keyword: Option<Keyword>,
    array_type_variant: Option<TypeVariant>,
    array_size: Option<IntegerLiteral>,
    tuple_types: Vec<TypeVariant>,
    alias_identifier: Option<String>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_unit(&mut self) {
        self.is_unit = true;
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

    pub fn push_tuple_type(&mut self, value: TypeVariant) {
        self.tuple_types.push(value)
    }

    pub fn set_alias_identifier(&mut self, value: String) {
        self.alias_identifier = Some(value);
    }

    pub fn finish(mut self) -> Type {
        let location = self.location.take().expect("Missing location");
        let variant = if let Some(alias_identifier) = self.alias_identifier.take() {
            TypeVariant::new_alias(alias_identifier)
        } else if self.is_unit {
            TypeVariant::new_unit()
        } else if let Some(keyword) = self.keyword.take() {
            match keyword {
                Keyword::Bool => TypeVariant::new_boolean(),
                Keyword::U { bitlength } => TypeVariant::new_integer_unsigned(bitlength),
                Keyword::I { bitlength } => TypeVariant::new_integer_signed(bitlength),
                Keyword::Field => TypeVariant::new_field(),
                _ => panic!("Always is one of the type keywords above"),
            }
        } else if let Some(array_type) = self.array_type_variant.take() {
            let array_size: usize = self.array_size.take().expect("Missing array size").into();
            TypeVariant::new_array(array_type, array_size)
        } else if !self.tuple_types.is_empty() {
            TypeVariant::new_tuple(self.tuple_types)
        } else {
            panic!("Always checked by the branches above");
        };

        Type { location, variant }
    }
}
