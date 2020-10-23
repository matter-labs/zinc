//!
//! The type variant.
//!

use crate::tree::expression::tree::Tree as ExpressionTree;
use crate::tree::r#type::Type;

///
/// The syntax type variant which is parsed directly from the source code.
/// The most important and frequently used variant is `Alias`, which represents a path expression
/// resolved in the source code scope hierarchy.
///
/// This entity is converted into the semantic type during the semantic analysis.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    /// `()` in the source code.
    Unit,
    /// `bool` in the source code.
    Boolean,
    /// `u{N}` in the source code.
    IntegerUnsigned {
        /// The unsigned integer bitlength.
        bitlength: usize,
    },
    /// `i{N}` in the source code.
    IntegerSigned {
        /// The signed integer bitlength.
        bitlength: usize,
    },
    /// `field` in the source code.
    Field,
    /// `[{type}; {expression}]` in the source code.
    Array {
        /// The array element type.
        inner: Box<Type>,
        /// The array size expression.
        size: ExpressionTree,
    },
    /// `({type1}, {type2}, ...)` in the source code.
    Tuple {
        /// The tuple element types.
        inners: Vec<Type>,
    },
    /// `{namespace1}::{namespace2}::...::{type}<generic1, generic2, ...>` in the source code.
    Alias {
        /// The path expression, which points to an aliased type.
        path: ExpressionTree,
        /// The optional generic type arguments.
        generics: Option<Vec<Type>>,
    },
}

impl Variant {
    ///
    /// A shortcut constructor.
    ///
    pub fn unit() -> Self {
        Self::Unit
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn boolean() -> Self {
        Self::Boolean
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn integer(is_signed: bool, bitlength: usize) -> Self {
        if is_signed {
            Self::integer_signed(bitlength)
        } else {
            Self::integer_unsigned(bitlength)
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn integer_unsigned(bitlength: usize) -> Self {
        Self::IntegerUnsigned { bitlength }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn integer_signed(bitlength: usize) -> Self {
        Self::IntegerSigned { bitlength }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn field() -> Self {
        Self::Field
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn array(inner: Type, size: ExpressionTree) -> Self {
        Self::Array {
            inner: Box::new(inner),
            size,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn tuple(inners: Vec<Type>) -> Self {
        Self::Tuple { inners }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn alias(path: ExpressionTree, generics: Option<Vec<Type>>) -> Self {
        Self::Alias { path, generics }
    }
}
