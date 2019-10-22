//!
//! The type builder.
//!

use crate::lexical::IntegerLiteral;
use crate::lexical::Keyword;
use crate::lexical::Location;
use crate::syntax::Identifier;
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
    tuple_has_comma: bool,
    alias_identifier: Option<Identifier>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_unit_if_empty(&mut self) {
        if self.tuple_types.is_empty() {
            self.is_unit = true;
        }
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

    pub fn set_tuple_comma(&mut self) {
        self.tuple_has_comma = true;
    }

    pub fn set_alias_identifier(&mut self, value: Identifier) {
        self.alias_identifier = Some(value);
    }

    pub fn finish(mut self) -> Type {
        let location = self.location.take().expect("Missing location");

        let variant = if let Some(identifier) = self.alias_identifier.take() {
            TypeVariant::new_alias(identifier.name)
        } else if let Some(keyword) = self.keyword.take() {
            match keyword {
                Keyword::Bool => TypeVariant::new_boolean(),
                Keyword::U { bitlength } => TypeVariant::new_integer_unsigned(bitlength),
                Keyword::I { bitlength } => TypeVariant::new_integer_signed(bitlength),
                Keyword::Field => TypeVariant::new_field(),
                _ => panic!("Always checked by the branches above"),
            }
        } else if let Some(array_type) = self.array_type_variant.take() {
            TypeVariant::new_array(array_type, self.array_size.take().expect("Missing array size"))
        } else if !self.tuple_types.is_empty() {
            if !self.tuple_has_comma {
                self.tuple_types.pop().expect("Always contains an element")
            } else {
                TypeVariant::new_tuple(self.tuple_types)
            }
        } else if self.is_unit {
            TypeVariant::new_unit()
        } else {
            panic!("Always checked by the branches above");
        };

        Type { location, variant }
    }
}
