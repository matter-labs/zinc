//!
//! The type builder.
//!

use crate::lexical::token::lexeme::keyword::Keyword;
use crate::lexical::token::location::Location;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
use crate::syntax::tree::r#type::variant::Variant as TypeVariant;
use crate::syntax::tree::r#type::Type;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    is_unit: bool,
    keyword: Option<Keyword>,
    array_type: Option<Type>,
    array_size: Option<ExpressionTree>,
    tuple_element_types: Vec<Type>,
    path_expression: Option<ExpressionTree>,
}

static BUILDER_TYPE_INVALID_KEYWORD: &str =
    "The type builder has got an unexpected non-type keyword: ";
static VALIDATED_BY_THE_TYPE_PARSER: &str =
    "Unreachable as long as the type parser works correctly";

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_unit_if_empty(&mut self) {
        if self.tuple_element_types.is_empty() {
            self.is_unit = true;
        }
    }

    pub fn set_keyword(&mut self, value: Keyword) {
        self.keyword = Some(value);
    }

    pub fn set_array_type(&mut self, value: Type) {
        self.array_type = Some(value);
    }

    pub fn set_array_size_expression(&mut self, value: ExpressionTree) {
        self.array_size = Some(value);
    }

    pub fn push_tuple_element_type(&mut self, value: Type) {
        self.tuple_element_types.push(value)
    }

    pub fn set_path_expression(&mut self, value: ExpressionTree) {
        self.path_expression = Some(value);
    }

    pub fn finish(mut self) -> Type {
        let location = self
            .location
            .take()
            .unwrap_or_else(|| panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "location"));

        let variant = if let Some(path) = self.path_expression.take() {
            TypeVariant::alias(path)
        } else if let Some(keyword) = self.keyword.take() {
            match keyword {
                Keyword::Bool => TypeVariant::boolean(),
                Keyword::IntegerUnsigned { bitlength } => TypeVariant::integer_unsigned(bitlength),
                Keyword::IntegerSigned { bitlength } => TypeVariant::integer_signed(bitlength),
                Keyword::Field => TypeVariant::field(),
                keyword => panic!("{}{}", BUILDER_TYPE_INVALID_KEYWORD, keyword),
            }
        } else if let Some(array_type) = self.array_type.take() {
            TypeVariant::array(
                array_type,
                self.array_size.take().unwrap_or_else(|| {
                    panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "array size")
                }),
            )
        } else if !self.tuple_element_types.is_empty() {
            TypeVariant::tuple(self.tuple_element_types)
        } else if self.is_unit {
            TypeVariant::unit()
        } else {
            panic!(VALIDATED_BY_THE_TYPE_PARSER);
        };

        Type::new(location, variant)
    }
}
