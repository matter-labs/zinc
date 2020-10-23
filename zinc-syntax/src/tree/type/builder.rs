//!
//! The type builder.
//!

use zinc_lexical::Keyword;
use zinc_lexical::Location;

use crate::tree::expression::tree::Tree as ExpressionTree;
use crate::tree::r#type::variant::Variant as TypeVariant;
use crate::tree::r#type::Type;

///
/// The type builder.
///
#[derive(Default)]
pub struct Builder {
    /// The location of the syntax construction.
    location: Option<Location>,
    /// The unit type, which means that the type is an empty tuple, which is the same as unit.
    is_unit: bool,
    /// The type keyword, which means that the type is intrinsic.
    keyword: Option<Keyword>,
    /// The array type, which means that the type is an array.
    array_type: Option<Type>,
    /// The array size expression, which means that the type is an array.
    array_size: Option<ExpressionTree>,
    /// The tuple elements, which means that the type is a tuple.
    tuple_element_types: Vec<Type>,
    /// The path expression, which means that the type is an alias.
    path_expression: Option<ExpressionTree>,
    /// The optional generic type arguments.
    generics: Option<Vec<Type>>,
}

/// The invalid type keyword panic, which is prevented by the type parser.
static BUILDER_TYPE_INVALID_KEYWORD: &str =
    "The type builder has got an unexpected non-type keyword: ";
/// The unreachable branch panic, which is prevented by the type parser.
static VALIDATED_BY_THE_TYPE_PARSER: &str =
    "Unreachable as long as the type parser works correctly";

impl Builder {
    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_unit_if_empty(&mut self) {
        if self.tuple_element_types.is_empty() {
            self.is_unit = true;
        }
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_keyword(&mut self, value: Keyword) {
        self.keyword = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_array_type(&mut self, value: Type) {
        self.array_type = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_array_size_expression(&mut self, value: ExpressionTree) {
        self.array_size = Some(value);
    }

    ///
    /// Pushes the corresponding builder value.
    ///
    pub fn push_tuple_element_type(&mut self, value: Type) {
        self.tuple_element_types.push(value)
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_path_expression(&mut self, value: ExpressionTree) {
        self.path_expression = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_generics(&mut self, value: Vec<Type>) {
        self.generics = Some(value);
    }

    ///
    /// Finalizes the builder and returns the built value.
    ///
    /// # Panics
    /// If some of the required items has not been set.
    ///
    pub fn finish(mut self) -> Type {
        let location = self.location.take().unwrap_or_else(|| {
            panic!(
                "{}{}",
                zinc_const::panic::BUILDER_REQUIRES_VALUE,
                "location"
            )
        });

        let variant = if let Some(path) = self.path_expression.take() {
            TypeVariant::alias(path, self.generics.take())
        } else if let Some(keyword) = self.keyword.take() {
            match keyword {
                Keyword::Bool => TypeVariant::boolean(),
                Keyword::IntegerUnsigned { bitlength } => TypeVariant::integer_unsigned(bitlength),
                Keyword::IntegerSigned { bitlength } => TypeVariant::integer_signed(bitlength),
                Keyword::Field => TypeVariant::field(),
                keyword => panic!("{}{}", self::BUILDER_TYPE_INVALID_KEYWORD, keyword),
            }
        } else if let Some(array_type) = self.array_type.take() {
            TypeVariant::array(
                array_type,
                self.array_size.take().unwrap_or_else(|| {
                    panic!(
                        "{}{}",
                        zinc_const::panic::BUILDER_REQUIRES_VALUE,
                        "array size"
                    )
                }),
            )
        } else if !self.tuple_element_types.is_empty() {
            TypeVariant::tuple(self.tuple_element_types)
        } else if self.is_unit {
            TypeVariant::unit()
        } else {
            panic!(self::VALIDATED_BY_THE_TYPE_PARSER);
        };

        Type::new(location, variant)
    }
}
