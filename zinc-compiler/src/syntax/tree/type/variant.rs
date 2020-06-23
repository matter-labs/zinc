//!
//! The type variant.
//!

use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
use crate::syntax::tree::r#type::Type;

///
/// The syntax type variant which is parsed directly from the source code.
/// The most important variant is the `Alias` one, which represents a path expression which is
/// resolved in the source code scope hierarchy.
///
/// The type is converted to a next phase semantic type during the semantic analysis.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    /// `()` in the source code
    Unit,
    /// `bool` in the source code
    Boolean,
    /// `u{N}` in the source code
    IntegerUnsigned { bitlength: usize },
    /// `i{N}` in the source code
    IntegerSigned { bitlength: usize },
    /// `field` in the source code
    Field,
    /// `[ {type}; {expression} ]` in the source code
    Array {
        inner: Box<Type>,
        size: ExpressionTree,
    },
    /// `( {type1}, {type2}, ... )` in the source code
    Tuple { inners: Vec<Type> },
    /// `{namespace} :: {namespace} :: ... {type}` in the source code
    Alias { path: ExpressionTree },
}

impl Variant {
    pub fn unit() -> Self {
        Self::Unit
    }

    pub fn boolean() -> Self {
        Self::Boolean
    }

    pub fn integer(is_signed: bool, bitlength: usize) -> Self {
        if is_signed {
            Self::integer_signed(bitlength)
        } else {
            Self::integer_unsigned(bitlength)
        }
    }

    pub fn integer_unsigned(bitlength: usize) -> Self {
        Self::IntegerUnsigned { bitlength }
    }

    pub fn integer_signed(bitlength: usize) -> Self {
        Self::IntegerSigned { bitlength }
    }

    pub fn field() -> Self {
        Self::Field
    }

    pub fn array(inner: Type, size: ExpressionTree) -> Self {
        Self::Array {
            inner: Box::new(inner),
            size,
        }
    }

    pub fn tuple(inners: Vec<Type>) -> Self {
        Self::Tuple { inners }
    }

    pub fn alias(path: ExpressionTree) -> Self {
        Self::Alias { path }
    }
}
